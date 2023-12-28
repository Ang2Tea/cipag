use cipag::encrypt_implement::VigenereCipher;
use cipag::encrypt_core::{Decrypt, Encrypt};
use cipag::encrypt_core::error::{ErrorKind, ErrMessage};

const CRYPTO_TEXT: &str = "карл у клары украл кораллы";
const CRYPTO_KEY: &str = "кларнет";
const SHIFT_N: isize = 1;

const ENCRYPTED_TEXT_N0: &str = "хлрь б пюкьы дшхтц цобнрюё";
const ENCRYPTED_TEXT_N1: &str = "цмсэ в рялэь ещцуч чпвосяж";

const UNCORRECT_CRYPTO_KEY: &str = "клар нет";

#[test]
fn test_vigenere_encrypt_n0(){
    let crypto = VigenereCipher::new_with_key(CRYPTO_KEY)
    .unwrap();

    assert_eq!(
        Some(String::from(ENCRYPTED_TEXT_N0)), 
        crypto.encrypt_text(String::from(CRYPTO_TEXT))
    );
}

#[test]
fn test_vigenere_decrypt_n0(){
    let crypto = VigenereCipher::new_with_key(CRYPTO_KEY)
    .unwrap();

    assert_eq!(
        Some(String::from(CRYPTO_TEXT)), 
        crypto.decrypt_text(String::from(ENCRYPTED_TEXT_N0))
    );
}

#[test]
fn test_vigenere_encrypt_n1(){
    let crypto = VigenereCipher::new_with_shift_n(CRYPTO_KEY, SHIFT_N)
    .unwrap();

    assert_eq!(
        Some(String::from(ENCRYPTED_TEXT_N1)), 
        crypto.encrypt_text(String::from(CRYPTO_TEXT))
    );
}

#[test]
fn test_vigenere_decrypt_n1() {
    let crypto = VigenereCipher::new_with_shift_n(CRYPTO_KEY, SHIFT_N)
    .unwrap();

    assert_eq!(
        Some(String::from(CRYPTO_TEXT)), 
        crypto.decrypt_text(String::from(ENCRYPTED_TEXT_N1))
    );
}

#[test]
fn test_vigenere_uncorrect_key() {
    let crypto = VigenereCipher::new_with_key(UNCORRECT_CRYPTO_KEY);

    assert_eq!(
        Err(ErrorKind::InvalidKeyError(
            ErrMessage::new(
                String::from("The key has characters that are not found in the alphabet"),
                char::from(' ')
            )
        )),
        crypto
    );
}