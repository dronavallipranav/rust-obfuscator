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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_cipher_and_decrypt() {
        let key = "xnasff3wcedj";
        let test_strings = ["Hello", "World", "1234", "!@#$%^&*()"];

        for &original in &test_strings {
            let encrypted = xor_cipher(original, &key);
            let decrypted = labyrinth_helper::decrypt_string(&encrypted, &key);
            assert_eq!(original, decrypted, "Failed for string: {}", original);
        }
    }
    #[test]
    fn test_xor_cipher_and_decrypt_customkey() {
        //set key
        std::env::set_var("LABYRINTH_KEY", "testkey");
        //test loc from encrypt_string meant to extract key
        let key = env::var("LABYRINTH_KEY").unwrap_or_else(|_| "xnasff3wcedj".to_string());
        assert_eq!(key, "testkey");

        let test_strings = ["Hello", "World", "1234", "!@#$%^&*()"];

        for &original in &test_strings {
            let encrypted = xor_cipher(original, &key);
            let decrypted = labyrinth_helper::decrypt_string(&encrypted, &key);
            assert_eq!(original, decrypted, "Failed for string: {}", original);
        }
    }
}
