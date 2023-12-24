pub trait Encrypt{
    fn encrypt(self, text: String) -> Option<String>;
}

pub trait Decrypt{
    fn decrypt(self, encrypt_text: String) -> Option<String>;
}