//integration tests testing whole crate
#[test]
fn test_encrypt_macro() {
    let decrypted = cryptify::encrypt_string!("Hello World");
    assert_eq!("Hello World", decrypted);
}

#[test]
fn test_flow_macro() {
    //manually test for now with cargo expand
    cryptify::flow_stmt!();
    assert_eq!(1, 1);
}
