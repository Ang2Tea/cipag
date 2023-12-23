use std::{isize, char, result};
use crate::encrypt_abstract::encryption::{Encrypt, Decrypt};

struct VigenereCipher {
    key: String,
    alphabet: String,
    shift_n: isize,
}


impl VigenereCipher{
    pub fn new(key: &String) -> Self {
        Self{
            key: *key,
            alphabet: "абвгдеёжзийклмнопрстуфхцчшщъыьэюя".to_string(),
            shift_n: 0,
        }
    }
    pub fn set_key(&self, key: &String) {
        self.key = *key;
    }
    pub fn set_alphabet(&self, alphabet: &String) {
        self.alphabet = *alphabet;
    }
    pub fn set_shift_n(&self, shift_n: isize) {
        self.shift_n = shift_n;
    }
    fn base_crypt(self, text: String, crypt_diff: fn(isize, isize, isize) -> isize) -> String {
        let mut result = String::new();
        if !text.len() < 1 {return Err("Пустой текст".to_string());}

        for item in text.chars().into_iter() {

            if !self.alphabet.contains(item) {
                result.push(item);
                continue;
            }

            return Ok(result);
        }
    }
}

impl Encrypt for VigenereCipher {
    fn encrypt(self, text: String) -> String {
        return self.base_crypt(text, |a: isize, b: isize, len: isize| -> isize { a + b })
    }
}

impl Decrypt for VigenereCipher {
    fn decrypt(self, encrypt_text: String) -> String {
        return self.base_crypt(encrypt_text, |a: isize, b: isize, len: isize| -> isize { a + len - b });
    }
}