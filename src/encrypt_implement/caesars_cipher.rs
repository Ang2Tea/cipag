use std::{isize, char};

use crate::encrypt_core::{Encrypt, Decrypt};
use crate::encrypt_core::error::{ErrorKind, ErrMessage};
use crate::encrypt_implement::DEFAULT_ALPHABET;

#[derive(Debug, PartialEq)]
pub struct CaesarsCipher {
    alphabet: String,
    count_alphabet: usize,
    rot: isize,
}

impl CaesarsCipher {
    /// Creates a new [`CaesarsCipher`] with alphabet.
    pub fn new(rot: isize, alphabet: &str) -> Self {
        Self{
            alphabet: alphabet
            .to_lowercase(),
            count_alphabet: alphabet
            .to_lowercase()
            .chars()
            .count(),
            rot,
        }
    }
    /// Creates a new [`CaesarsCipher`] without alphabet.
    pub fn new_with_shift_n(shift_n: isize) -> Self {
        return Self::new(shift_n, DEFAULT_ALPHABET);
    }
    /// Sets the alphabet of this [`CaesarsCipher`].
    pub fn set_alphabet(&mut self, alphabet: String) {
        self.alphabet = alphabet.to_lowercase();
        self.count_alphabet = self.alphabet.chars().count();
    }
    /// Sets the shift n of this [`CaesarsCipher`].
    pub fn set_shift_n(&mut self, rot: isize) {
        self.rot = rot;
    }

    fn base_crypt_char<T>(&self, char: char, crypt_diff: T) -> Result<char, ErrorKind>
    where T: Fn(isize, isize) -> usize {
        let temp_char = char.to_ascii_lowercase();
        
        let item_index = self.alphabet
        .chars()
        .position(|c| c == temp_char)
        .ok_or(ErrorKind::CharError(
            ErrMessage::new(
                String::from("Item not find for alphabet"),
                temp_char
            )
        ))? as isize;

        let rot_index = crypt_diff(item_index, self.rot) % self.count_alphabet;

        return self.alphabet
        .chars()
        .nth(rot_index)
        .ok_or(ErrorKind::IndexError(
            ErrMessage::new(
                String::from("Error get char for index"),
                rot_index
            )
        ));
    }

    fn base_crypt_text<T>(&self, text: String, crypt_diff: T) -> Result<String, ErrorKind>
    where T: Fn(isize, isize) -> usize {
        let mut result = String::new();

        for item in text.chars() {
            if !self.alphabet.contains(item.to_ascii_lowercase()) {
                result.push(item);
                continue;
            }

            result.push(self.base_crypt_char(item, &crypt_diff)?);
        }

        Ok(result)
    }
}

impl Encrypt for CaesarsCipher {
    fn encrypt_text(&self, encrypted_text: String) -> Option<String> {
        return self.base_crypt_text(
            encrypted_text, 
            |a, b| (a + b) as usize
        )
        .ok();
    }
}

impl Decrypt for CaesarsCipher {
    fn decrypt_text(&self, decrypted_text: String) -> Option<String> {
        return self.base_crypt_text(
            decrypted_text, 
            |a, b| (a + (self.count_alphabet as isize) - b) as usize 
        )
        .ok();
    }
}