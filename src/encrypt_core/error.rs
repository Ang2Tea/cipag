#[derive(Debug, PartialEq)]
pub struct ErrMessage<T>{
    message: String,
    value: T
}

impl<T> ErrMessage<T> {
    pub fn new( message: String, value: T) -> Self {
        Self {
            message: message,
            value: value
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    IndexError(ErrMessage<usize>),
    CharError(ErrMessage<char>),
    InvalidKeyError(ErrMessage<char>)
}