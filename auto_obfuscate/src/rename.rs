use rand::{ Rng };
use syn::{
    visit_mut::VisitMut,
    parse_file,
    Ident,
    ItemFn,
    Local,
    Expr,
    ExprPath,
    Macro,
    Visibility,
    UseTree,
    UsePath,
    UseName,
    UseRename,
    ItemUse,
};
use quote::quote;
use std::collections::{ HashMap, HashSet };
use proc_macro2::{ TokenStream, TokenTree, Group };

#[cfg(test)]
mod rename_tests;

#[derive(Clone)]
pub struct RenameConfig {
    pub enable_rename_obfuscation: bool,
}

//default rename to false
impl RenameConfig {
    pub fn default() -> Self {
        Self {
            enable_rename_obfuscation: false,
        }
    }
}

pub struct VariableRenamer {
    renamed_vars: HashMap<String, String>,
    imported_functions: HashSet<String>,
    pub enabled: bool,
}

impl VariableRenamer {
    pub fn new(config: RenameConfig) -> Self {
        VariableRenamer {
            renamed_vars: HashMap::new(),
            imported_functions: HashSet::new(),
            enabled: config.enable_rename_obfuscation,
        }
    }
    //helper to process Macros tokenstream and check if it is an identifier or another macro or func call
    fn process_tokens(&mut self, tokens: TokenStream) -> TokenStream {
        tokens
            .into_iter()
            .map(|token| {
                match token {
                    TokenTree::Group(group) => {
                        let modified_tokens = self.process_tokens(group.stream());
                        TokenTree::Group(Group::new(group.delimiter(), modified_tokens))
                    }
                    TokenTree::Ident(ident) => {
                        if let Some(new_name) = self.renamed_vars.get(&ident.to_string()) {
                            TokenTree::Ident(Ident::new(new_name, ident.span()))
                        } else {
                            TokenTree::Ident(ident)
                        }
                    }
                    _ => token,
                }
            })
            .collect()
    }

    //scan use statements to identify imported functions and add them to blacklist
    fn identify_imported_functions(&mut self, tree: &UseTree) {
        match tree {
            UseTree::Path(UsePath { ident: _, tree, .. }) => {
                self.identify_imported_functions(tree);
            }
            UseTree::Name(UseName { ident }) => {
                self.imported_functions.insert(ident.to_string());
            }
            UseTree::Rename(UseRename { rename, .. }) => {
                self.imported_functions.insert(rename.to_string());
            }
            _ => {}
        }
    }
    pub fn rename(&mut self, code: &str) -> String {
        let ast = parse_file(code).expect("Failed to parse code");
        let mut modified_ast = ast.clone();
        self.visit_file_mut(&mut modified_ast);
        let modified_code = quote!(#modified_ast).to_string();
        modified_code
    }
}

//check to see if the function is local, only rename local functions for now
fn is_local_function(fn_item: &ItemFn) -> bool {
    !matches!(fn_item.vis, Visibility::Public(_))
}

impl VisitMut for VariableRenamer {
    //visit use statements to identify imported functions
    fn visit_item_use_mut(&mut self, i: &mut ItemUse) {
        self.identify_imported_functions(&i.tree);
        syn::visit_mut::visit_item_use_mut(self, i);
    }

    fn visit_item_fn_mut(&mut self, i: &mut ItemFn) {
        //rename function names unless it's main
        let old_name = i.sig.ident.to_string();
        if
            old_name != "main" &&
            is_local_function(i) &&
            !self.imported_functions.contains(&old_name)
        {
            if !self.renamed_vars.contains_key(&old_name) {
                let new_name = random_name();
                self.renamed_vars.insert(old_name.clone(), new_name.clone());
                i.sig.ident = Ident::new(&new_name, i.sig.ident.span());
            }
        }

        let len = i.block.stmts.len();
        for (index, stmt) in i.block.stmts.iter_mut().enumerate() {
            match stmt {
                syn::Stmt::Local(local) => self.visit_local_mut(local),

                //check if last statement is an expression
                syn::Stmt::Expr(expr, _) if index == len - 1 => {
                    self.visit_expr_mut(expr);
                }

                _ => syn::visit_mut::visit_stmt_mut(self, stmt),
            }
        }
    }

    fn visit_macro_mut(&mut self, i: &mut Macro) {
        i.tokens = self.process_tokens(i.tokens.clone());
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
            }
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
            }
            //handle function call
            Expr::Call(expr_call) => {
                //rename function names
                if let Expr::Path(expr_path) = &mut *expr_call.func {
                    if let Some(last_segment) = expr_path.path.segments.last_mut() {
                        let func_name = last_segment.ident.to_string();
                        if let Some(new_name) = self.renamed_vars.get(&func_name) {
                            last_segment.ident = Ident::new(new_name, last_segment.ident.span());
                        }
                    }
                }
                //rename all function arguments
                for arg in &mut expr_call.args {
                    self.visit_expr_mut(arg);
                }
            }

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

// Function to generate a random name
fn random_name() -> String {
    let mut rng = rand::thread_rng();
    let name_length = rng.gen_range(3..=10);

    let mut last_char_was_underscore = false;
    let mut name = String::new();

    while name.len() < name_length {
        let next_char = if rng.gen_bool(0.8) { rng.gen_range(b'a'..=b'z') as char } else { '_' };

        // Ensure not two underscores in a row
        if !(last_char_was_underscore && next_char == '_') {
            name.push(next_char);
            last_char_was_underscore = next_char == '_';
        }
    }
    // Ensure the name does not start or end with an underscore
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
