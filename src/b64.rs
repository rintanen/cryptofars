pub fn encode(bytes: &[u8]) -> String {
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

pub fn decode(base64: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut buffer = 0;
    let mut buffer_length = 0;

    for &byte in base64.as_bytes() {
        let value = match byte {
            b'A'..=b'Z' => byte - 65,
            b'a'..=b'z' => byte - 97 + 26,
            b'0'..=b'9' => byte - 48 + 52,
            b'+' => 62,
            b'/' => 63,
            b'=' => continue,
            _ => panic!("Invalid base64 character: {}", byte),
        };

        // Add the value to the buffer and update buffer length:
        // 1. Left shift the current contents of the buffer by 6 bits to make room for the new 6-bit value.
        // 2. Use bitwise OR to combine the shifted buffer and the new 6-bit value.
        //    This effectively adds the 6-bit value to the rightmost side of the buffer.
        // 3. Increment the buffer length by 6 to reflect the addition of the new bits to the buffer.
        buffer = (buffer << 6) | value as u32;
        buffer_length += 6;


        // If there are enough bits in the buffer, extract a byte and reset buffer
        if buffer_length >= 8 {
            buffer_length -= 8;
            bytes.push((buffer >> buffer_length) as u8);
            buffer &= (1 << buffer_length) - 1;
        }
    }
    bytes
}
