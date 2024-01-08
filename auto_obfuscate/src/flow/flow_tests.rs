#[cfg(test)]
use super::*;

#[test]
fn test_loop_insertion() {
    let code =
        r#"
            fn main() {
                let mut num1 = 10;
                let num2 = 20;
                num1 = 30;
                let sum = num1 + num2;
                println!("The sum is: {}", sum);
            }
        "#;
    let flow_config = FlowConfig {
        enable_flow_obfuscation: true,
        use_macro: false,
    };
    let mut obfuscator = FlowObfuscator::new(flow_config);
    let modified_code = obfuscator.flow_obfuscate(code);

    assert_ne!(modified_code, code);
    //check if loop is inserted at start of block
    assert!(modified_code.contains("_is_dummy_145"), "Dummy loop not found in modified code");
    //valid rust code
    let parse_result = syn::parse_file(&modified_code);
    assert!(parse_result.is_ok(), "Modified code is not valid Rust code");
}

#[test]
fn test_loop_skip() {
    let code =
        r#"
        fn calculate_sum(num1: i32, num2: i32) -> i32 {
            num1 + num2
        }
        fn ok(){
            
        }
            fn main() {
                let mut num1 = 10;
                let num2 = 20;
                num1 = 30;
                let sum = num1 + num2;
                println!("The sum is: {}", sum);
            }
        "#;
    let flow_config = FlowConfig {
        enable_flow_obfuscation: true,
        use_macro: false,
    };
    let mut obfuscator = FlowObfuscator::new(flow_config);
    let modified_code = obfuscator.flow_obfuscate(code);

    assert_ne!(modified_code, code);
    //check if loop is inserted at start of block
    assert!(modified_code.contains("_is_dummy_145"), "Dummy loop not found in modified code");

    let parse_result = syn::parse_file(&modified_code);
    assert!(parse_result.is_ok(), "Modified code is not valid Rust code");

    let num_loops = modified_code.matches("_is_dummy_145").count();
    assert!(num_loops == 2, "exactly two dummy loops not found in modified code");
}
