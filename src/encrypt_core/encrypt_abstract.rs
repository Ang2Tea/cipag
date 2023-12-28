pub trait Encrypt{
    fn encrypt_text(&self, encrypted_text: String) -> Option<String>;
    fn encrypt_char(&self, encrypted_char: char, key_char: char) -> Option<char>;
}

pub trait Decrypt{
    fn decrypt_text(&self, decrypted_text: String) -> Option<String>;
    fn decrypt_char(&self, decrypted_char: char, key_char: char) -> Option<char>;
}