//! `labyrinth_macros` crate provides procedural macros for compile-time obfuscation. NOT MEANT TO BE USED STANDALONE.
//!
//! This crate includes macros like `encrypt_string` and `flow_stmt` which are used
//! to enhance the security of Rust code by obfuscating strings and control flows.
use proc_macro::TokenStream;
use quote::quote;
use syn::*;
use std::env;
use rand::Rng;
use rand::seq::SliceRandom;

/// A procedural macro that adds a compile-time randomly generated loop and variables.
///
/// # Note
/// The unsafe operation is meant to help the dummy loop survive compiler optimizations. only writes to dummy variable
///
#[proc_macro]
pub fn flow_stmt(_: TokenStream) -> TokenStream {
    let mut rng = rand::thread_rng();

    let initial_value = rng.gen_range(1..=10);
    let increment_value = rng.gen_range(1..=4);
    let add_extra_dummy_variable = rng.gen_bool(0.5);

    let mut statements = vec![
        quote! { let mut _dummy_counter = #initial_value; },
        quote! { let _dummy_increment = #increment_value; },
        quote! { let _dummy_upper_bound = 100; }
    ];

    //add random dummy variable occasionally
    if add_extra_dummy_variable {
        let extra_dummy_value = rng.gen_range(1..=10);
        statements.push(quote! { let _extra_dummy_var = #extra_dummy_value; });
    }

    //randomize the order of variable assignments
    statements.shuffle(&mut rng);

    let loop_block =
        quote! {
        loop {
            if _dummy_counter > _dummy_upper_bound {
                break;
            }
            //prevent compiler optimizations
            unsafe {
                std::ptr::write_volatile(&mut _dummy_counter, _dummy_counter + _dummy_increment);
            }
        }
    };

    let generated_loop =
        quote! {
        {
            let _is_dummy_145 = true;
            #(#statements)*
            #loop_block
        }
    };

    TokenStream::from(generated_loop)
}
/// A procedural macro that encrypts a string literal at compile time.
///
/// # Parameters
/// - `input`: The string literal to be encrypted.
///
#[proc_macro]
pub fn encrypt_string(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let string = input.value();

    //set key to seeded env key or default
    let key = env::var("CRYPTIFY_KEY").unwrap_or_else(|_| "xnasff3wcedj".to_string());

    let encrypted_string = xor_cipher(&string, &key);

    let output = quote! {
        cryptify::decrypt_string(#encrypted_string).as_ref()
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

//for self-contained tests
#[allow(dead_code)]
fn decrypt_string(encrypted: &str) -> String {
    let key = std::env::var("CRYPTIFY_KEY").unwrap_or_else(|_| "xnasff3wcedj".to_string());
    encrypted
        .chars()
        .zip(key.chars().cycle())
        .map(|(encrypted_char, key_char)| ((encrypted_char as u8) ^ (key_char as u8)) as char)
        .collect()
}

//unit tests testing decryption logic
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_cipher_and_decrypt() {
        let key = "xnasff3wcedj";
        let test_strings = ["Hello", "World", "1234", "!@#$%^&*()"];

        for &original in &test_strings {
            let encrypted = xor_cipher(original, &key);
            let decrypted = decrypt_string(&encrypted);
            assert_eq!(original, decrypted, "Failed for string: {}", original);
        }
    }
    #[test]
    fn test_xor_cipher_and_decrypt_customkey() {
        //set key
        std::env::set_var("CRYPTIFY_KEY", "testkey");
        //test loc from encrypt_string meant to extract key
        let key = env::var("CRYPTIFY_KEY").unwrap_or_else(|_| "xnasff3wcedj".to_string());
        assert_eq!(key, "testkey");

        let test_strings = ["Hello", "World", "1234", "!@#$%^&*()"];
        for &original in &test_strings {
            let encrypted = xor_cipher(original, &key);
            let decrypted = decrypt_string(&encrypted);
            assert_eq!(original, decrypted, "Failed for string: {}", original);
        }
    }
}
