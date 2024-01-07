use labyrinth;

#[test]
fn test_encrypt_macro() {
    let decrypted = labyrinth::encrypt_string!("Hello World");
    assert_eq!("Hello World", decrypted);
}
