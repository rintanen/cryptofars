type Deciphered = (u8, Vec<u8>, u32);

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

fn find_key_and_text(bytes: &[u8]) -> Deciphered {
    let mut max_score = 0;
    let mut text: Vec<u8> = Vec::new();
    let mut correct_key = 0;
    for key in 0..=u8::MAX {
        let result = xor_single_byte(&bytes, key);
        let score = score_by_character_frequency(&result);
        if score > max_score {
            text = result;
            max_score = score;
            correct_key = key;
        }
    }
    (correct_key, text, max_score)
}

fn main() {
    // TASK 3
    let hex = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let bytes = hex::decode(hex).unwrap();
    let (key, text_bytes, _) = find_key_and_text(&bytes);
    println!("TASK 3\nkey: {} \ntext: {}", key as char, std::str::from_utf8(&text_bytes).unwrap());

    // TASK 4
    let file = include_str!("../../inputs/04.in");
    let (key, text_bytes) = file
        .lines()
        .map(|l| {
            let bytes = hex::decode(l).unwrap();
            find_key_and_text(&bytes)
        })
        .max_by_key(|(_, _, score)| *score)
        .map(|(key, text_bytes, _)| (key, text_bytes))
        .unwrap();
    println!("TASK 4\nkey: {} \ntext: {}", key as char, std::str::from_utf8(&text_bytes).unwrap());
}
