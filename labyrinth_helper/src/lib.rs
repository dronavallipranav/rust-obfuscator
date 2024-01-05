//helper function to decrypt string at runtime
pub fn decrypt_string(encrypted: &str, key: &str) -> String {
    encrypted
        .chars()
        .zip(key.chars().cycle())
        .map(|(encrypted_char, key_char)| ((encrypted_char as u8) ^ (key_char as u8)) as char)
        .collect()
}
