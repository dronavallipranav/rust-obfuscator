#[test]
fn test_encrypt_macro() {
    let decrypted = labyrinth::encrypt_string!("Hello World");
    assert_eq!("Hello World", decrypted);
}

#[test]
fn test_flow_macro() {
    //manually test for now with cargo expand
    labyrinth::flow_stmt!();
    assert_eq!(1, 1);
}
