use base64::{Engine as _, engine::{general_purpose}};
use openssl::symm::{decrypt, Cipher};

fn main() {
    let file = include_str!("../../inputs/07.in");
    let file = file.replace("\n", "");
    let bytes = match general_purpose::STANDARD.decode(file.as_bytes()) {
        Ok(bytes) => bytes,
        Err(e) => panic!("Error: {}", e),
    };
    let key = "YELLOW SUBMARINE".as_bytes();
    let cipher = Cipher::aes_128_ecb();
    let decrypted = decrypt(cipher, key, None, &bytes).unwrap();
    println!("{}", std::str::from_utf8(&decrypted).unwrap());
}