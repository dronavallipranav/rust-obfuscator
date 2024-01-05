use labyrinth::encrypt_string;
use syn::{ visit_mut::VisitMut, Block, Stmt, parse_quote, parse_file, Expr, Pat, PatIdent };
use quote::quote;

struct StringObfuscator;

impl VisitMut for StringObfuscator {}
