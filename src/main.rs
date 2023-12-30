extern crate rand;
extern crate syn;

use rand::{Rng};
use syn::{visit_mut::VisitMut, Ident, ItemFn, Local, Expr, ExprPath, Macro};
use quote::quote;
use proc_macro2::TokenStream;
use std::collections::HashMap;
use proc_macro2::TokenTree;

// Function to generate a random name
fn random_name() -> String {
    let mut rng = rand::thread_rng();
    let name_length = rng.gen_range(3..=10);
    
    let mut name = (0..name_length)
        .map(|_| {
            if rng.gen_bool(0.8) { 
                rng.gen_range(b'a'..=b'z') as char
            } else { 
                '_'
            }
        })
        .collect::<String>();
    //make sure name doesn't end or start in underscore
    if name.starts_with('_') {
        name.remove(0);
        name.insert(0, rng.gen_range(b'a'..=b'z') as char);
    }
    if name.ends_with('_') {
        name.pop();
        name.push(rng.gen_range(b'a'..=b'z') as char);
    }

    name
}

struct VariableRenamer{
    renamed_vars: HashMap<String, String>,
}

impl VariableRenamer {
    fn new() -> Self {
        VariableRenamer {
            renamed_vars: HashMap::new(),
        }
    }
}

impl VisitMut for VariableRenamer {

    fn visit_item_fn_mut(&mut self, i: &mut ItemFn) {
        let len = i.block.stmts.len();
        for (index, stmt) in i.block.stmts.iter_mut().enumerate() {
            match stmt {
            
                syn::Stmt::Local(local) => self.visit_local_mut(local),
                
                //check if last statement is an expression
                syn::Stmt::Expr(expr, _) if index == len - 1 => {
                    self.visit_expr_mut(expr);
                },

                _ => syn::visit_mut::visit_stmt_mut(self, stmt),
            }
        }
    
    }

    fn visit_macro_mut(&mut self, i: &mut Macro) {
 
     i.tokens = i.tokens.clone().into_iter().map(|token| {

        if let TokenTree::Ident(ref ident) = token {
            let ident_str = ident.to_string();
            // Check if macro params are in changed variables
            if let Some(new_name) = self.renamed_vars.get(&ident_str) {
                return TokenTree::Ident(Ident::new(new_name, ident.span()));
            }
        }
        token
    }).collect();
    }

    fn visit_expr_mut(&mut self, expr: &mut Expr) {

        //check expression type and rename vars accordingly
        match expr {
            Expr::Path(ExprPath { ref mut path, .. }) => {
                if let Some(last_segment) = path.segments.last_mut() {
                    let var_name = last_segment.ident.to_string();
                    if let Some(new_name) = self.renamed_vars.get(&var_name) {
                        last_segment.ident = Ident::new(new_name, last_segment.ident.span());
                    }
                }
            },
            Expr::Assign(expr_assign) => {
                //check left of expression
                if let Expr::Path(ExprPath { ref mut path, .. }) = *expr_assign.left {
                    if let Some(last_segment) = path.segments.last_mut() {
                        let var_name = last_segment.ident.to_string();
                        if let Some(new_name) = self.renamed_vars.get(&var_name) {
                            last_segment.ident = Ident::new(new_name, last_segment.ident.span());
                        }
                    }
                }
                // recursively visit right of assignment in case of more complex expression
                self.visit_expr_mut(&mut *expr_assign.right);
            },
            //handle function call
            Expr::Call(expr_call) => {
    
                for arg in &mut expr_call.args {
                    self.visit_expr_mut(arg);
                }

            },

            _ => {}
        }

        syn::visit_mut::visit_expr_mut(self, expr);
    }

    //visit local variables
    fn visit_local_mut(&mut self, local: &mut Local) {

        if let Some(local_init) = &mut local.init {
            self.visit_expr_mut(local_init.expr.as_mut());
        }
        //change variable name
        if let syn::Pat::Ident(ref mut pat_ident) = local.pat {
            let old_name = pat_ident.ident.to_string();
            let new_name = random_name();
            self.renamed_vars.insert(old_name, new_name.clone());
            pat_ident.ident = Ident::new(&new_name, pat_ident.ident.span());
        }
    }
   
}

fn main() {
    let code = r#"
        fn calculate_sum(a: i32, b: i32) -> i32 {
            let result = a + b;
            result
        }

        fn main() {
            let mut num1 = 10;
            let num2 = 20;
            num1 = 30;
            let sum = calculate_sum(num1, num2);
            println!("The sum is: {}", sum);
        }
    "#;

    let ast = syn::parse_file(code).expect("Unable to parse code");

    //modify ast with new variable names
    let mut visitor = VariableRenamer::new();
    let mut modified_ast = ast.clone();
    visitor.visit_file_mut(&mut modified_ast);
    let new_code: TokenStream = quote! { #modified_ast };

    println!("{}", new_code);
}