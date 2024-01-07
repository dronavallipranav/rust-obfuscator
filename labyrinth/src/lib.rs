pub use labyrinth_macros::*;
//helper function to decrypt string at runtime
pub fn decrypt_string(encrypted: &str) -> String {
    let key = std::env::var("LABYRINTH_KEY").unwrap_or_else(|_| "xnasff3wcedj".to_string());
    encrypted
        .chars()
        .zip(key.chars().cycle())
        .map(|(encrypted_char, key_char)| ((encrypted_char as u8) ^ (key_char as u8)) as char)
        .collect()
}
