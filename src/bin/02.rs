extern crate hex;

fn xor_bytes(bytes1: &[u8], bytes2: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    for (byte1, byte2) in bytes1.iter().zip(bytes2.iter()) {
        result.push(byte1 ^ byte2);
    }
    result
}


fn main() {
    let bytes = hex::decode("1c0111001f010100061a024b53535009181c").unwrap();
    let key = hex::decode("686974207468652062756c6c277320657965").unwrap();

    let result = xor_bytes(&bytes, &key);

    let result_hex = hex::encode(result);

    println!("{}", result_hex);
    assert_eq!(result_hex, "746865206b696420646f6e277420706c6179".to_string());
}