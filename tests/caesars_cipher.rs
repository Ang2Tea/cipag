use cipag::encrypt_implement::CaesarsCipher;
use cipag::encrypt_core::{Decrypt, Encrypt};

const CRYPTO_TEXT: &str = "карл у клары украл кораллы";
const SHIFT_N3: isize = 3;
const SHIFT_N5: isize = 5;
const ENCRYPTED_TEXT_N3: &str = "нгуо ц ногую цнуго нсугоою";
const ENCRYPTED_TEXT_N5: &str = "пехр ш преха шпхер пухерра";

#[test]
fn test_caesars_encrypt_n3(){
    let crypto = CaesarsCipher::new_with_shift_n(SHIFT_N3);

    assert_eq!(
        Some(String::from(ENCRYPTED_TEXT_N3)), 
        crypto.encrypt_text(String::from(CRYPTO_TEXT))
    );
}

#[test]
fn test_caesars_decrypt_n3(){
    let crypto = CaesarsCipher::new_with_shift_n(SHIFT_N3);

    assert_eq!(
        Some(String::from(CRYPTO_TEXT)), 
        crypto.decrypt_text(String::from(ENCRYPTED_TEXT_N3))
    );
}

#[test]
fn test_caesars_encrypt_n5(){
    let crypto = CaesarsCipher::new_with_shift_n(SHIFT_N5);

    assert_eq!(
        Some(String::from(ENCRYPTED_TEXT_N5)), 
        crypto.encrypt_text(String::from(CRYPTO_TEXT))
    );
}

#[test]
fn test_caesars_decrypt_n5() {
    let crypto = CaesarsCipher::new_with_shift_n(SHIFT_N5);

    assert_eq!(
        Some(String::from(CRYPTO_TEXT)), 
        crypto.decrypt_text(String::from(ENCRYPTED_TEXT_N5))
    );
}

#[test]
fn test_caesars_encrypt_uncorrect_keн() {
    let crypto = CaesarsCipher::new_with_shift_n(104);

    assert_eq!(
        Some(String::from(ENCRYPTED_TEXT_N5)), 
        crypto.encrypt_text(String::from(CRYPTO_TEXT))
    );
}

#[test]
fn test_caesars_decrypt_uncorrect_keн() {
    let crypto = CaesarsCipher::new_with_shift_n(104);

    assert_eq!(
        Some(String::from(CRYPTO_TEXT)), 
        crypto.decrypt_text(String::from(ENCRYPTED_TEXT_N5))
    );
}