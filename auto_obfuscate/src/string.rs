use syn::{
    visit_mut::VisitMut,
    visit::Visit,
    parse_file,
    parse_str,
    Expr,
    ExprLit,
    LitStr,
    Lit,
    File,
    Local,
    parse_quote,
};
use quote::quote;
use proc_macro2::{ TokenStream, TokenTree };

#[cfg(test)]
mod string_tests;

#[derive(Clone)]
pub struct StringConfig {
    pub enable_string_obfuscation: bool,
    pub percentage: u8,
}
impl StringConfig {
    pub fn default() -> Self {
        Self {
            enable_string_obfuscation: true,
            percentage: 100,
        }
    }
}

pub struct StringObfuscator {
    pub enabled: bool,
    #[allow(dead_code)]
    percentage: u8,
    encrypted_count: usize,
    strings_to_encrypt: usize,
}

impl StringObfuscator {
    pub fn new(config: StringConfig) -> Self {
        Self {
            enabled: config.enable_string_obfuscation,
            percentage: config.percentage,
            encrypted_count: 0,
            strings_to_encrypt: 0,
        }
    }
    #[allow(dead_code)]
    fn process_macro_tokens(&self, tokens: TokenStream) -> TokenStream {
        tokens
            .into_iter()
            .map(|token| {
                (
                    match token {
                        TokenTree::Literal(lit) => {
                            //convert literal obj to string
                            let lit_str = lit.to_string();

                            //replace literal obj with macro call
                            if let Ok(lit_str) = parse_str::<LitStr>(&lit_str) {
                                let macro_call: TokenStream =
                                    quote! {
                        cryptify::encrypt_string!(#lit_str)
                    };
                                return macro_call;
                            }

                            TokenTree::Literal(lit)
                        }
                        //handle nested groups in macro
                        TokenTree::Group(group) => {
                            let new_stream = self.process_macro_tokens(group.stream());
                            TokenTree::Group(proc_macro2::Group::new(group.delimiter(), new_stream))
                        }
                        _ => token,
                    }
                ).into()
            })
            .collect()
    }

    pub fn obfuscate_strings(&mut self, code: &str) -> String {
        let ast = parse_file(code).expect("Failed to parse code");

        let total_strings = count_string_literals(&ast);
        let strings_to_encrypt = (
            ((self.percentage as f32) / 100.0) *
            (total_strings as f32)
        ).ceil() as usize;
        self.encrypted_count = 0;
        self.strings_to_encrypt = strings_to_encrypt;

        let mut modified_ast = ast.clone();
        self.visit_file_mut(&mut modified_ast);
        let modified_code = quote!(#modified_ast).to_string();
        modified_code
    }
}

impl VisitMut for StringObfuscator {
    //replace all string literals with call to obfuscation macro
    fn visit_local_mut(&mut self, local: &mut Local) {
        if let Some(local_init) = &mut local.init {
            //match on local variables that contain string literal assignments
            if let Expr::Lit(ExprLit { lit: Lit::Str(lit_str), .. }) = &*local_init.expr {
                let encrypted = quote! { cryptify::encrypt_string!(#lit_str) };
                let new_expr: Expr = parse_quote!(#encrypted);
                *local_init.expr = *Box::new(new_expr);
            }
        }

        syn::visit_mut::visit_local_mut(self, local);
    }
}

struct StringLiteralCounter {
    count: usize,
}

impl StringLiteralCounter {
    fn new() -> Self {
        Self { count: 0 }
    }
}

impl<'ast> Visit<'ast> for StringLiteralCounter {
    fn visit_lit_str(&mut self, _lit_str: &'ast LitStr) {
        self.count += 1;
    }
}

fn count_string_literals(ast: &File) -> usize {
    let mut counter = StringLiteralCounter::new();
    counter.visit_file(ast);
    counter.count
}
