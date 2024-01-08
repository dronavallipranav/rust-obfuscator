use syn::{ visit_mut::VisitMut, parse_file, parse2, parse_str, Expr, LitStr, Lit, Macro };
use quote::quote;
use proc_macro2::{ TokenStream, TokenTree };

#[cfg(test)]
mod string_tests;

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
    percentage: u8,
}

impl StringObfuscator {
    pub fn new(config: StringConfig) -> Self {
        Self {
            enabled: config.enable_string_obfuscation,
            percentage: config.percentage,
        }
    }
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
                        labyrinth::encrypt_string!(#lit_str)
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
        let mut modified_ast = ast.clone();
        self.visit_file_mut(&mut modified_ast);
        let modified_code = quote!(#modified_ast).to_string();
        modified_code
    }
}

impl VisitMut for StringObfuscator {
    //replace all string literals with call to obfuscation macro
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        if self.enabled == false {
            return;
        }
        if let Expr::Lit(expr_lit) = expr {
            if let Lit::Str(_) = &expr_lit.lit {
                //replace string literal with macro call
                let macro_call =
                    quote! {
                    labyrinth::encrypt_string!(#expr_lit)
                };

                //replace expression to use macro call
                *expr = parse2(macro_call).expect("Failed to parse macro call");
            }
        }

        syn::visit_mut::visit_expr_mut(self, expr);
    }
    fn visit_macro_mut(&mut self, mac: &mut Macro) {
        if self.enabled == false {
            return;
        }
        //check to see if macro is not obfuscation macro
        if
            mac.path.segments.len() == 2 &&
            mac.path.segments[0].ident == "labyrinth" &&
            mac.path.segments[1].ident == "encrypt_string"
        {
            return;
        }
        //encrypt string literal within macro
        let new_tokens = self.process_macro_tokens(mac.tokens.clone());
        mac.tokens = new_tokens;
    }
}
