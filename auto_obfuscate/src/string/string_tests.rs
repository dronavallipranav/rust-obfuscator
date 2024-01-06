#[cfg(test)]
use super::*;

#[test]
fn test_replacement_in_macro() {
    let code = r#"
        fn main() {
            println!("Hello, world!");
        }
    "#;
    let mut string_obfuscator = StringObfuscator::new();
    let obfuscated_code = string_obfuscator.obfuscate_strings(code);
    assert_ne!(code, obfuscated_code);
    assert!(obfuscated_code.contains("encrypt_string"));

    let parse_result = syn::parse_file(&obfuscated_code);
    assert!(parse_result.is_ok(), "Modified code is not valid Rust code");
}

#[test]
fn test_replacement_in_expr() {
    let code = r#"
        fn main() {
            let b = "Hello, world!";
        }
    "#;
    let mut string_obfuscator = StringObfuscator::new();
    let obfuscated_code = string_obfuscator.obfuscate_strings(code);
    assert_ne!(code, obfuscated_code);
    assert!(obfuscated_code.contains("encrypt_string"));

    let parse_result = syn::parse_file(&obfuscated_code);
    assert!(parse_result.is_ok(), "Modified code is not valid Rust code");
}

#[test]
fn test_replacement_in_nested_macro() {
    let code =
        r#"
    fn main() {
        let num1 = 10;
        let num2 = 20;
        println!("Formatted: {}", format!("Num1: {}, Num2: {}", num1, num2));
    }
"#;

    let mut string_obfuscator = StringObfuscator::new();
    let obfuscated_code = string_obfuscator.obfuscate_strings(code);
    assert_ne!(code, obfuscated_code);
    assert!(obfuscated_code.contains("encrypt_string"));

    let parse_result = syn::parse_file(&obfuscated_code);
    assert!(parse_result.is_ok(), "Modified code is not valid Rust code");
}
