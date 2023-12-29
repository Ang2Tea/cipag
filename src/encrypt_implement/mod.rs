mod vigenere_cipher;
mod caesars_cipher;
const DEFAULT_ALPHABET: &str = "абвгдеёжзийклмнопрстуфхцчшщъыьэюя";

pub use vigenere_cipher::VigenereCipher;
pub use caesars_cipher::CaesarsCipher;