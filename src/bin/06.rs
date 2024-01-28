use base64::{Engine as _, alphabet, engine::{self, general_purpose}};
use itertools::Itertools;

type Deciphered = (u8, Vec<u8>, u32);

fn repeating_key_xor(bytes: &[u8], key: &[u8]) -> Vec<u8> {
    bytes
        .iter()
        .zip(key.iter().cycle())
        .map(|(byte, key_byte)| byte ^ key_byte)
        .collect()
}

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


fn hamming_distance(bytes1: &[u8], bytes2: &[u8]) -> usize {
    let mut distance = 0;
    for (byte1, byte2) in bytes1.iter().zip(bytes2.iter()) {
        let mut xor = byte1 ^ byte2;
        while xor > 0 {
            distance += xor & 1;
            xor >>= 1;
        }
    }
    distance as usize
}


fn find_keysize(s: &[u8]) -> usize {
    (2..=40).min_by_key(|size| {
        let blocks = [
            &s[size*0..size*1],
            &s[size*1..size*2],
            &s[size*2..size*3],
            &s[size*3..size*4],
        ];
        let avg_dist = blocks.iter()
            .tuple_combinations()
            .map(|(b1,b2)| hamming_distance(*b1, *b2))
            .sum::<usize>();
        avg_dist / size
    }).unwrap()
}


fn transpose_blocks(bytes: &[u8], blocksize: usize) -> Vec<Vec<u8>> {
    let mut transposed_blocks: Vec<Vec<u8>> = Vec::new();
    for _ in 0..blocksize {
        transposed_blocks.push(Vec::new());
    }

    for block in bytes.chunks(blocksize) {
        for (i, byte) in block.iter().enumerate() {
            transposed_blocks[i].push(*byte);
        }
    }
    transposed_blocks
}

fn main() {
    let file = include_str!("../../inputs/06.in");
    let file = file.replace("\n", "");

    let bytes = match general_purpose::STANDARD.decode(file.as_bytes()) {
        Ok(bytes) => bytes,
        Err(e) => panic!("Error: {}", e),
    };

    let keysize = find_keysize(&bytes);

    let transposed_blocks = transpose_blocks(&bytes, keysize);

    let key: Vec<u8> = transposed_blocks
        .iter()
        .map(|block| {
            let (key, _, _) = find_key_and_text(&block);
            key
        })
        .collect();

    let text = repeating_key_xor(&bytes, &key);

    println!("key: {} \ntext: {}", std::str::from_utf8(&key).unwrap(), std::str::from_utf8(&text).unwrap());
}