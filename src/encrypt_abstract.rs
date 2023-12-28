pub trait Encrypt{
    fn encrypt(&self, text: String) ->Result<String, ErrorKind>;
}

pub trait Decrypt{
    fn decrypt(&self, encrypt_text: String) -> Result<String, ErrorKind>;
}

pub enum ErrorKind {
    IndexErr(String),
    CharErr(String),
    IteratorErr(String)
}