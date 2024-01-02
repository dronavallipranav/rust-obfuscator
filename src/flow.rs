use syn::{ visit_mut::VisitMut, Block, Stmt, parse_quote, parse_file, Expr, Pat };
use quote::quote;
struct Obfuscator;

#[cfg(test)]
mod flow_tests;

macro_rules! dummy_loop {
    () => {
        {
            let _is_dummy_145 = true;
            let mut _dummy_counter = 0;
            let _dummy_upper_bound = rand::thread_rng().gen_range(10..=100);
            loop {
                let _dummy_break_chance = rand::thread_rng().gen_range(0..=100);
                if _dummy_break_chance < 5 || _dummy_counter > _dummy_upper_bound {
                    break;
                }
                _dummy_counter += 1;
            }
        }
    };
}

impl Obfuscator {
    fn flow_obfuscate(&mut self, code: &str) -> String {
        let ast = parse_file(code).expect("Failed to parse code");
        let mut modified_ast = ast.clone();
        self.visit_file_mut(&mut modified_ast);
        let modified_code = quote!(#modified_ast).to_string();
        modified_code
    }
    //check to see if statement in block is dummy loop
    fn is_dummy_loop(stmt: &Stmt) -> bool {
        if let Stmt::Expr(Expr::Block(expr_block)) = stmt {
            for stmt in &expr_block.block.stmts {
                if let Stmt::Local(local) = stmt {
                    if let Pat::Ident(PatIdent { ident, .. }) = &local.pat {
                        if ident == "_is_dummy_145" {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }
}

impl VisitMut for Obfuscator {
    fn visit_block_mut(&mut self, block: &mut Block) {
        //check if the block already contains the dummy loop
        if block.stmts.iter().any(|stmt| Self::is_dummy_loop(stmt)) {
            return;
        }

        //loop that seemingly runs forever but breaks randomly or with upperbound
        let dummy_loop: Stmt = parse_quote! {
            {
                dummy_loop!();
            }
            //insert loop at start of every block
            block.stmts.insert(0, dummy_loop);

            syn::visit_mut::visit_block_mut(self, block);
        };
    }
}
