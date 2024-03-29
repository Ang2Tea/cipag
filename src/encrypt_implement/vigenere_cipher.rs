use std::{isize, char};

use crate::encrypt_core::{Encrypt, Decrypt};
use crate::encrypt_core::error::{ErrorKind, ErrMessage};

const DEFAULT_ALPHABET: &str = "абвгдеёжзийклмнопрстуфхцчшщъыьэюя";
const SHIFT_N: isize = 0;

#[derive(Debug, PartialEq)]
pub struct VigenereCipher {
    key: String,
    alphabet: String,
    count_alphabet: usize,
    shift_n: isize,
}


impl VigenereCipher {
    /// Creates a new [`VigenereCipher`] with alphabet.
    pub fn new(key: &str, alphabet: &str, shift_n: isize) -> Self {
        Self{
            key: key.to_lowercase(),
            alphabet: alphabet
            .to_lowercase(),
            count_alphabet: alphabet
            .to_lowercase()
            .chars()
            .count(),
            shift_n,
        }
    }
    /// Creates a new [`VigenereCipher`] with alphabet.
    pub fn new_with_alphabet(key: &str, alphabet: &str) -> Self {
        return Self::new(key, alphabet, SHIFT_N);
    }
    /// Creates a new [`VigenereCipher`] without alphabet.
    pub fn new_with_shift_n(key: &str, shift_n: isize) -> Self {
        return Self::new(key, DEFAULT_ALPHABET, shift_n);
    }
    /// Creates a new [`VigenereCipher`] without alphabet.
    pub fn new_with_key(key: &str) -> Self {
        return Self::new(key, DEFAULT_ALPHABET, SHIFT_N);
    }
    
    /// Sets the key of this [`VigenereCipher`].
    pub fn set_key(&mut self, key: String) {
        self.key = key.to_lowercase();
    }
    /// Sets the alphabet of this [`VigenereCipher`].
    pub fn set_alphabet(&mut self, alphabet: String) {
        self.alphabet = alphabet.to_lowercase();
        self.count_alphabet = self.alphabet.chars().count();
    }
    /// Sets the shift n of this [`VigenereCipher`].
    pub fn set_shift_n(&mut self, shift_n: isize) {
        self.shift_n = shift_n;
    }

    fn base_crypt_char<T>(&self, char: char, key_char: char, crypt_diff: T) -> Result<char, ErrorKind>
    where T: Fn(isize, isize) -> usize {
        let temp_char = char.to_ascii_lowercase();
        
        let item_index = get_char_index(&self.alphabet, temp_char)? as isize;
        let key_index = get_char_index(&self.alphabet, key_char)? as isize;

        let shit_n_index = crypt_diff(
            crypt_diff(item_index, key_index) as isize, 
            self.shift_n
        ) % self.count_alphabet;

        return self.alphabet
        .chars()
        .nth(shit_n_index)
        .ok_or(ErrorKind::IndexError(
            ErrMessage::new(
                String::from("Error get char for index"),
                shit_n_index
            )
        ));

        fn get_char_index(collection: &str, item: char) -> Result<usize, ErrorKind> {
            collection
            .chars()
            .position(|c| c == item)
            .ok_or(ErrorKind::CharError(
                ErrMessage::new(
                    String::from("Item not find for alphabet"),
                    item
                )
            ))
        }
    }

    fn base_crypt_text<T>(&self, text: String, crypt_diff: T) -> Result<String, ErrorKind>
    where T: Fn(isize, isize) -> usize {
        let mut result = String::new();

        let mut key_iter = self.key
        .chars();

        for item in text.chars() {
            let temp_key_char = key_iter
            .clone()
            .next()
            .unwrap_or(self.key.chars().next().unwrap());

            let mut next_key_iter = || -> char {
                key_iter
                .next()
                .unwrap_or_else( || {
                    key_iter = self.key.chars();
                    key_iter.next().unwrap()
                })
            };
            
            if !self.alphabet.contains(temp_key_char) {
                _ = next_key_iter();
            }

            if !self.alphabet.contains(item.to_ascii_lowercase()) {
                result.push(item);
                continue;
            }

            let key_char = next_key_iter();
            result.push(self.base_crypt_char(item, key_char, &crypt_diff)?);
        }

        Ok(result)
    }
}

impl Encrypt for VigenereCipher {
    fn encrypt_text(&self, encrypted_text: String) -> Option<String> {
        return self.base_crypt_text(
            encrypted_text, 
            |a, b| (a + b) as usize
        )
        .ok();
    }

    fn encrypt_char(&self, encrypted_char: char, key_char: char) -> Option<char> {
        return self.base_crypt_char(
            encrypted_char, 
            key_char,
            |a, b| (a + b) as usize
        )
        .ok();
    }
}

impl Decrypt for VigenereCipher {
    fn decrypt_text(&self, decrypted_text: String) -> Option<String> {
        return self.base_crypt_text(
            decrypted_text, 
            |a, b| (a + (self.count_alphabet as isize) - b) as usize 
        )
        .ok();
    }

    fn decrypt_char(&self, decrypted_char: char, key_char: char) -> Option<char> {
        return self.base_crypt_char(
            decrypted_char, 
            key_char,
            |a, b| (a + (self.count_alphabet as isize) - b) as usize 
        )
        .ok();
    }
}