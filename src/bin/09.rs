fn pkcs7_padding(bytes: &[u8], block_size: usize) -> Vec<u8> {
    let mut result = bytes.to_vec();
    let padding = block_size - (bytes.len() % block_size);
    for _ in 0..padding {
        result.push(padding as u8);
    }
    result
}


fn main() {
    let bytes = b"YELLOW SUBMARINE";
    let result = pkcs7_padding(bytes, 20);
    println!("{:?}", result);
}