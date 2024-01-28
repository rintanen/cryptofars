fn repeating_key_xor(bytes: &[u8], key: &[u8]) -> Vec<u8> {
    bytes
        .iter()
        .zip(key.iter().cycle())
        .map(|(byte, key_byte)| byte ^ key_byte)
        .collect()
}

fn main() {
    let text = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key = "ICE";

    let encrypted = repeating_key_xor(text.as_bytes(), key.as_bytes());
    let encrypted_hex = hex::encode(encrypted);

    println!("{}", encrypted_hex);
}