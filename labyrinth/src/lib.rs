extern crate proc_macro;
extern crate labyrinth_helper;

use proc_macro::TokenStream;
use quote::quote;
use syn::*;
use std::env;

#[proc_macro]
pub fn encrypt_string(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let string = input.value();

    //set key to seeded env key or default
    let key = env::var("LABYRINTH_KEY").unwrap_or_else(|_| "xnasff3wcedj".to_string());

    let encrypted_string = xor_cipher(&string, &key);

    let output = quote! {
        labyrinth_helper::decrypt_string(#encrypted_string, #key)
    };

    TokenStream::from(output)
}

fn xor_cipher(input: &str, key: &str) -> String {
    input
        .chars()
        .zip(key.chars().cycle())
        .map(|(input_char, key_char)| { ((input_char as u8) ^ (key_char as u8)) as char })
        .collect()
}
