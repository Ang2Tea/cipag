pub trait Encrypt{
    fn encrypt_text(&self, encrypted_text: String) -> Option<String>;
}

pub trait Decrypt{
    fn decrypt_text(&self, decrypted_text: String) -> Option<String>;
}