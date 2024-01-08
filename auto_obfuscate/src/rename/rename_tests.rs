#[cfg(test)]
use super::*;
use regex::Regex;

//function for testing
fn is_valid_rust_var_name(name: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
    re.is_match(name)
}

#[test]
fn test_variable_renamer() {
    let code =
        r#"
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
    let rename_config = RenameConfig {
        enable_rename_obfuscation: true,
    };
    let mut renamer = VariableRenamer::new(rename_config);
    let modified_code = renamer.rename(code);

    //compare the modified code with the original
    assert_ne!(modified_code, code);

    //check if names used are all valid rust variable names
    for new_name in renamer.renamed_vars.values() {
        assert!(is_valid_rust_var_name(new_name), "Invalid variable name: {}", new_name);
    }

    //original names should not be found in modified code (except for sum TO DO: remove when string encryption is implemented)
    let original_names = vec!["calculate_sum", "result", "num1", "num2"];
    for name in original_names {
        assert!(
            !modified_code.contains(name),
            "Original name '{}' still found in modified code",
            name
        );
    }
    let parse_result = syn::parse_file(&modified_code);
    assert!(parse_result.is_ok(), "Modified code is not valid Rust code");
}
#[test]
fn test_nested_function_calls() {
    let code =
        r#"
            fn add_one(x: i32) -> i32 {
                x + 1
            }

            fn calculate_sum(a: i32, b: i32) -> i32 {
                let result = a + b;
                result
            }

            fn main() {
                let mut num1 = 10;
                let num2 = 20;
                num1 = 30;
                let sum = calculate_sum(add_one(num1), num2);
                println!("The sum is: {}", sum);
            }
        "#;
    let rename_config = RenameConfig {
        enable_rename_obfuscation: true,
    };
    let mut renamer = VariableRenamer::new(rename_config);
    let modified_code = renamer.rename(code);

    let original_names = vec!["calculate_sum", "add_one", "num1", "num2", "result"];
    for name in original_names {
        assert!(
            !modified_code.contains(name),
            "Original function name '{}' still found in modified code",
            name
        );
    }
    let parse_result = syn::parse_file(&modified_code);
    assert!(parse_result.is_ok(), "Modified code is not valid Rust code");
}

#[test]
fn test_nested_macros() {
    let code =
        r#"
        fn main() {
            let num1 = 10;
            let num2 = 20;
            println!("Formatted: {}", format!("Num1: {}, Num2: {}", num1, num2));
        }
    "#;
    let rename_config = RenameConfig {
        enable_rename_obfuscation: true,
    };
    let mut renamer = VariableRenamer::new(rename_config);
    let modified_code = renamer.rename(code);

    let original_names = vec!["calculate_sum", "add_one", "num1", "num2", "result"];
    for name in original_names {
        assert!(
            !modified_code.contains(name),
            "Original function name '{}' still found in modified code",
            name
        );
    }
    let parse_result = syn::parse_file(&modified_code);
    assert!(parse_result.is_ok(), "Modified code is not valid Rust code");
}
#[test]
fn test_user_defined_nested_macro() {
    let code =
        r#"
            macro_rules! identity {
                ($x:expr) => ($x)
            }

            fn main() {
                let num1 = 10;
                let num2 = 20;
                println!("Num1: {}", identity!(num1));
                println!("Num2: {}", identity!(num2));
            }
        "#;
    let rename_config = RenameConfig {
        enable_rename_obfuscation: true,
    };
    let mut renamer = VariableRenamer::new(rename_config);
    let modified_code = renamer.rename(code);

    let original_names = vec!["calculate_sum", "add_one", "num1", "num2", "result"];
    for name in original_names {
        assert!(
            !modified_code.contains(name),
            "Original function name '{}' still found in modified code",
            name
        );
    }
    let parse_result = syn::parse_file(&modified_code);
    assert!(parse_result.is_ok(), "Modified code is not valid Rust code");
}
#[test]
fn test_function_in_macro() {
    let code =
        r#"
            fn add_one(x: i32) -> i32 {
                x + 1
            }

            fn main() {
                let num = 10;
                println!("Num + 1: {}", add_one(num));
            }
        "#;
    let rename_config = RenameConfig {
        enable_rename_obfuscation: true,
    };
    let mut renamer = VariableRenamer::new(rename_config);
    let modified_code = renamer.rename(code);

    let original_names = vec!["calculate_sum", "add_one", "num1", "num2", "result"];
    for name in original_names {
        assert!(
            !modified_code.contains(name),
            "Original function name '{}' still found in modified code",
            name
        );
    }
    let parse_result = syn::parse_file(&modified_code);
    assert!(parse_result.is_ok(), "Modified code is not valid Rust code");
}
