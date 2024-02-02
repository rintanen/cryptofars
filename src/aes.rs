// enum Cipher {
//     Aes128Cbc,
//     Aes128Ctr,
//     Aes128Ecb,
//     Aes192Cbc,
//     Aes192Ctr,
//     Aes192Ecb,
//     Aes256Cbc,
//     Aes256Ctr,
//     Aes256Ecb,
// }
//
// pub fn encrypt(key: &[u8], iv: &[u8], bytes: &[u8]) -> Vec<u8> {
//     let cipher = Cipher::aes_128_cbc();
//     encrypt(cipher, key, Some(iv), bytes).unwrap()
// }
//
// pub fn decrypt(key: &[u8], iv: &[u8], bytes: &[u8]) -> Vec<u8> {
//     let cipher = Cipher::aes_128_cbc();
//     decrypt(cipher, key, Some(iv), bytes).unwrap()
// }
//
//
// fn aes_ecb_decrypt(block: &[u8], key: &[u8]) -> Vec<u8> {
//     let mut encrypter = Crypter::new(Cipher::aes_128_ecb(), Mode::Decrypt, key, None).unwrap();
//     encrypter.pad(false);
//     let mut out = vec![0;block.len()+16];
//     encrypter.update(&block, &mut out).unwrap();
//     out.truncate(block.len());
//     out
// }
//
//
// fn aes_cbc_decrypt(bytes: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
//     let mut result = Vec::new();
//     let mut prev_block = iv.to_vec();
//     let mut blocks = bytes.chunks(16);
//     while let Some(block) = blocks.next() {
//         let decrypted = aes_ecb_decrypt(block, key);
//         let xored = xor(&decrypted, &prev_block);
//         result.extend_from_slice(&xored);
//         prev_block = block.to_vec();
//     }
//     result
// }