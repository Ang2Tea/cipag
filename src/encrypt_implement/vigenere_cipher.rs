use std::{isize, char};

use crate::encrypt_abstract::{Encrypt, Decrypt};

pub struct VigenereCipher {
    key: String,
    alphabet: String,
    shift_n: isize,
}


impl VigenereCipher{
    /// Creates a new [`VigenereCipher`].
    pub fn new(key: String) -> Self {
        Self{
            key: key.to_lowercase(),
            alphabet: "абвгдеёжзийклмнопрстуфхцчшщъыьэюя".to_lowercase(),
            shift_n: 0,
        }
    }
    /// Sets the key of this [`VigenereCipher`].
    pub fn set_key(&mut self, key: String) {
        self.key = key.to_lowercase();
    }
    /// Sets the alphabet of this [`VigenereCipher`].
    pub fn set_alphabet(&mut self, alphabet: String) {
        self.alphabet = alphabet.to_lowercase();
    }
    /// Sets the shift n of this [`VigenereCipher`].
    pub fn set_shift_n(&mut self, shift_n: isize) {
        self.shift_n = shift_n;
    }

    fn base_crypt(&self, text: String, crypt_diff: fn(usize, isize, isize) -> usize) -> Option<String> {
        let temp_text = text.to_lowercase();
        let mut result = String::new();

        let count_alphabet = self.alphabet
        .chars()
        .count();

        let mut key_iter = self.key
        .chars();

        for item in temp_text.chars() {
            {
                let mut temp_key_iter = key_iter
                .clone();
                let check_key_item = temp_key_iter
                .nth(0)
                .unwrap_or_else(|| {
                    temp_key_iter = self.key.chars();
                    temp_key_iter.next().unwrap()
                });
    
                if !self.alphabet.contains(item) || !self.alphabet.contains(check_key_item) {
                    result.push(item);
                    continue;
                }
            }

            let key_index = get_char_index(&self.alphabet, key_iter
                .next()
                .unwrap_or_else( || {
                    key_iter = self.key.chars();
                    key_iter.next().unwrap()
                })
            )? as isize;
            let item_index = get_char_index(&self.alphabet, item)? as isize;

            let shit_n_index = crypt_diff(count_alphabet, crypt_diff(count_alphabet, item_index, key_index) as isize, self.shift_n) % count_alphabet;

            result.push(self.alphabet
                .chars()
                .nth(shit_n_index)?
            );

            fn get_char_index(collection: &String, item: char) -> Option<usize> {
                collection
                .chars()
                .position(|c| c == item)
            }
        }

        Some(result)
    }
}

impl Encrypt for VigenereCipher {
    fn encrypt(&self, text: String) -> Option<String> {
        return self.base_crypt(text, |_: usize, a: isize, b: isize| -> usize { (a + b) as usize });
    }
}

impl Decrypt for VigenereCipher {
    fn decrypt(&self, encrypt_text: String) -> Option<String> {
        return self.base_crypt(encrypt_text, |len: usize, a: isize, b: isize| -> usize { (a + (len as isize) - b) as usize });
    }
}