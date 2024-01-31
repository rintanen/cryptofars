use base64::{Engine as _, engine::{general_purpose}};
use openssl::symm::{decrypt, Cipher, Crypter, Mode};

fn xor(bytes1: &[u8], bytes2: &[u8]) -> Vec<u8> {
    bytes1
        .iter()
        .zip(bytes2)
        .map(|(b1,b2)| b1^b2)
        .collect()
}


fn aes_ecb_decrypt(block: &[u8], key: &[u8]) -> Vec<u8> {
    let mut encrypter = Crypter::new(Cipher::aes_128_ecb(), Mode::Decrypt, key, None).unwrap();
    encrypter.pad(false);
    let mut out = vec![0;block.len()+16];
    encrypter.update(&block, &mut out).unwrap();
    out.truncate(block.len());
    out
}


fn aes_cbc_decrypt(bytes: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut prev_block = iv.to_vec();
    let mut blocks = bytes.chunks(16);
    while let Some(block) = blocks.next() {
        let decrypted = aes_ecb_decrypt(block, key);
        let xored = xor(&decrypted, &prev_block);
        result.extend_from_slice(&xored);
        prev_block = block.to_vec();
    }
    result
}


fn main() {
    let input = include_str!("../../inputs/10.in");
    let input = input.replace("\n", "");
    let bytes = general_purpose::STANDARD.decode(input.as_bytes()).unwrap();

    let key = b"YELLOW SUBMARINE";
    let decrypted = aes_cbc_decrypt(&bytes, key, &[0; 16]);

    println!("{}", std::str::from_utf8(&decrypted).unwrap());

}