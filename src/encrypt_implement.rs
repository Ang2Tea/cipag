use std::isize;
use crate::encrypt_abstract::{Encrypt, Decrypt};

struct VigenereCipher {
    key: String,
    alphabet: String,
    shift_n: isize,
}


impl VigenereCipher{
    /// Creates a new [`VigenereCipher`].
    pub fn new(key: String) -> Self {
        Self{
            key: key,
            alphabet: "абвгдеёжзийклмнопрстуфхцчшщъыьэюя".to_string(),
            shift_n: 0,
        }
    }
    /// Sets the key of this [`VigenereCipher`].
    pub fn set_key(&mut self, key: String) {
        self.key = key;
    }
    /// Sets the alphabet of this [`VigenereCipher`].
    pub fn set_alphabet(&mut self, alphabet: String) {
        self.alphabet = alphabet;
    }
    /// Sets the shift n of this [`VigenereCipher`].
    pub fn set_shift_n(&mut self, shift_n: isize) {
        self.shift_n = shift_n;
    }

    fn base_crypt(self, text: String, crypt_diff: fn(usize, isize, isize) -> usize) -> Option<String> {

        let shift_n = self.shift_n.clone();
        let alphabet: &str = &self.alphabet;

        let mut result = String::new();

        if !text.len() < 1 {"".to_string();}

        let mut key_iter: usize = 0;
        for item in text.chars() {

            if !self.alphabet.contains(item) {
                result.push(item);
                continue;
            }

            let item_index = alphabet.find(item)? as isize;

            let key_index = alphabet
            .find(self.key
                .chars()
                .nth(key_iter)?
            )? as isize;
            key_iter += 1;

            let search_index = crypt_diff(alphabet.len(), item_index, key_index) as isize;

            let shit_n_index: usize = crypt_diff(alphabet.len(), search_index, shift_n) % alphabet.len();

            

            result.push(alphabet
                .chars()
                .nth(shit_n_index)?
            );

            if key_iter >= self.key.len() { key_iter = 0; }
        }

        Some(result)
    }
}

impl Encrypt for VigenereCipher {
    fn encrypt(self, text: String) -> Option<String> {
        return self.base_crypt(text, |_: usize, a: isize, b: isize| -> usize { (a + b) as usize });
    }
}

impl Decrypt for VigenereCipher {
    fn decrypt(self, encrypt_text: String) -> Option<String> {
        return self.base_crypt(encrypt_text, |len: usize, a: isize, b: isize| -> usize { (a + (len as isize) - b) as usize });
    }
}