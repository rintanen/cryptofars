fn xor_single_byte(bytes: &[u8], key: u8) -> Vec<u8> {
    bytes.iter().map(|&byte| byte ^ key).collect()
}


fn score_by_character_frequency(bytes: &[u8]) -> u32 {
    bytes
        .iter()
        .map(|b| {
            match b {
                b'a' | b'A' => 8,
                b'e' | b'E' => 13,
                b't' | b'T' => 9,
                b'o' | b'O' => 8,
                b'i' | b'I' => 7,
                b'n' | b'N' => 7,
                b' ' => 18,
                b'.' | b',' | b'!' | b'?' => 5,
                _ => 1,
            }
        })
        .sum()
}


fn main() {
    let hex = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let bytes = hex::decode(hex).unwrap();

    let mut max_score = 0;
    let mut text = String::new();
    let mut correct_key = 0;
    for key in 0..=u8::MAX {
        let result = xor_single_byte(&bytes, key);
        let score = score_by_character_frequency(&result);
        if score > max_score {
            text = String::from_utf8(result).unwrap();
            max_score = score;
            correct_key = key;
        }
    }
    println!("key: {}, \ntext: {}", correct_key as char, text);
}