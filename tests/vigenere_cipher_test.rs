use cipag::encrypt_implement::VigenereCipher;
use cipag::encrypt_abstract::{Decrypt, Encrypt};

const CRYPTO_TEXT: &str = "карл у клары украл кораллы";
const CRYPTO_KEY: &str = "кларнет";
const UNCORRECT_CRYPTO_KEY1: &str = "клар нет";

const ENCRYPT_TEXT_N0: &str = "хлрь б пюкьы дшхтц цобнрюё";
const ENCRYPT_TEXT_N1: &str = "цмсэ в рялэь ещцуч чпвосяж";

#[test]
fn test_vigenere_encrypt_n0(){
    let crypto = VigenereCipher::new(String::from(CRYPTO_KEY));
    assert_eq!(
        Some(String::from(ENCRYPT_TEXT_N0)), 
        crypto.encrypt(String::from(CRYPTO_TEXT))
    );
}

#[test]
fn test_vigenere_decrypt_n0(){
    let crypto = VigenereCipher::new(String::from(CRYPTO_KEY));

    assert_eq!(
        Some(String::from(CRYPTO_TEXT)), 
        crypto.decrypt(String::from(ENCRYPT_TEXT_N0))
    );
}

#[test]
fn test_vigenere_encrypt_n1(){
    let mut crypto = VigenereCipher::new(String::from(CRYPTO_KEY));
    crypto.set_shift_n(1);

    assert_eq!(
        Some(String::from(ENCRYPT_TEXT_N1)), 
        crypto.encrypt(String::from(CRYPTO_TEXT))
    );
}

#[test]
fn test_vigenere_decrypt_n1() {
    let mut crypto = VigenereCipher::new(String::from(CRYPTO_KEY));
    crypto.set_shift_n(1);

    assert_eq!(
        Some(String::from(CRYPTO_TEXT)), 
        crypto.decrypt(String::from(ENCRYPT_TEXT_N1))
    );
}

#[test]
fn test_vigenere_key_n0() {
    let crypto = VigenereCipher::new(String::from(UNCORRECT_CRYPTO_KEY1));

    let encrypt = crypto.encrypt(String::from(CRYPTO_TEXT));
    assert_eq!(
        Some(String::from(ENCRYPT_TEXT_N0)), 
        encrypt
    );

    assert_eq!(
        Some(String::from(CRYPTO_TEXT)), 
        crypto.decrypt(encrypt.unwrap())
    );
}

#[test]
fn test_vigenere_key_n1() {
    let mut crypto = VigenereCipher::new(String::from(UNCORRECT_CRYPTO_KEY1));
    crypto.set_shift_n(1);

    let encrypt = crypto.encrypt(String::from(CRYPTO_TEXT));
    assert_eq!(
        Some(String::from(ENCRYPT_TEXT_N1)), 
        encrypt
    );

    assert_eq!(
        Some(String::from(CRYPTO_TEXT)), 
        crypto.decrypt(encrypt.unwrap())
    );
}