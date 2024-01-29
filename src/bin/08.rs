use itertools::Itertools;

fn main() {
    let input = include_str!("../../inputs/08.in");
    // Find a line in the input text where blocks of 32 bytes repeat
    let ecb_encrypted = input.lines()
        .find(|text| text.as_bytes()
            .chunks(32)
            .tuple_combinations()
            .any(|(a,b)| a == b)
        )
        .unwrap();

    // print all 32 byte blocks in the line
    for c in ecb_encrypted.as_bytes().chunks(32) {
        println!("{}", String::from_utf8(c.to_vec()).unwrap());
    }
}