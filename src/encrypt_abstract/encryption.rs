pub trait Encrypt{
    fn encrypt(self, text: String) -> String;
}

pub trait Decrypt{
    fn decrypt(self, encrypt_text: String) -> String;
}