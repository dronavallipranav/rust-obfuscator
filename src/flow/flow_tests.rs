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
                let sum = calculate_sum(num1, num2);
                println!("The sum is: {}", sum);
            }
        "#;

    let mut obfuscator = Obfuscator::new();
    let modified_code = obfuscator.flow_obfuscate(code);

    assert_ne!(modified_code, code);
    println!("{}", modified_code);
    //check if loop is inserted at start of block
    assert!(modified_code.contains("_is_dummy_145"), "Dummy loop not found in modified code");
}
