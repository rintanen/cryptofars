use cryptopals::b64;

fn hex_to_bytes(hex_string: &str) -> Vec<u8> {
    // Convert hexadecimal string to bytes
    hex_string
        .as_bytes()
        .chunks(2)
        .filter_map(|chunk| {
            if chunk.len() == 2 {
                let byte_str = std::str::from_utf8(chunk).expect("Invalid UTF-8");
                u8::from_str_radix(byte_str, 16).ok()
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let bytes = hex_to_bytes(hex);

    let base64_result = b64::encode(&bytes);
    println!("{}", base64_result);

    let decoded_bytes = b64::decode(&base64_result);
    let as_string = std::str::from_utf8(&decoded_bytes).unwrap();
    println!("{}", as_string);
}
