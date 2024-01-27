fn hex_to_base64(hex_string: &str) -> String {
    // Convert hexadecimal string to bytes
    let hex_bytes = hex_string
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
        .collect::<Vec<u8>>();

    let base64 = base64_encode(&hex_bytes);

    base64
}

fn base64_encode(bytes: &[u8]) -> String {
    let mut base64 = String::new();
    for chunk in bytes.chunks(3) {
        // convert 3-byte chunk to a 24-bit binary value
        let binary_value = chunk.iter().fold(0, |acc, &byte| acc << 8 | byte as u32);

        // iterate over 4 groups of 6-bit chunks in the 24-bit binary value
        for i in 0..4 {
            // shift the 24-bit binary value to the right by 6 bits * i
            let six_bits = (binary_value >> (18 - (i * 6))) & 0b111111;

            // map the 6-bit value to corresponding base64 character
            let base64_char = match six_bits {
                0..=25 => (six_bits + 65 - 0) as u8 as char,
                26..=51 => (six_bits + 97 - 26) as u8 as char,
                52..=61 => (six_bits + 48 - 52) as u8 as char,
                62 => '+' as u8 as char,
                63 => '/' as u8 as char,
                _ => panic!("Invalid six bits: {}", six_bits),
            };
            base64.push(base64_char);
        }
    }
    base64
}

fn main() {
    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let base64_result = hex_to_base64(hex);
    println!("{}", base64_result);
}
