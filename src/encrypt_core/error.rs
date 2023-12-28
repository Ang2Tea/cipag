#[derive(Debug, PartialEq)]
pub struct ErrorMessage<T>{
    message: String,
    value: T
}

impl<T> ErrorMessage<T> {
    pub fn new( message: String, value: T) -> Self {
        Self {
            message: message,
            value: value
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    IndexErr(ErrorMessage<usize>),
    CharErr(ErrorMessage<char>),
    IteratorErr(ErrorMessage<usize>)
}