use std::collections::HashMap;

pub fn word_pattern(pattern: String, s: String) -> bool {
    let mut byte_to_word: HashMap<u8, &str> = HashMap::new();
    let mut word_to_byte: HashMap<&str, u8> = HashMap::new();

    let mut ind = 0;
    let pattern_bytes = pattern.as_bytes();

    for word in s.split_whitespace() {
        if ind >= pattern_bytes.len() {
            return false;
        }

        let cur_byte = pattern_bytes[ind];

        let matched_word = byte_to_word.entry(cur_byte).or_insert(word);
        let matched_byte = word_to_byte.entry(word).or_insert(cur_byte);

        if cur_byte != *matched_byte || word != *matched_word {
            return false;
        }

        ind = ind + 1;
    }

    ind == pattern_bytes.len()
}

fn main() {
    let result = word_pattern(String::from("abb"), String::from("cat dog dog cat"));
    println!("{result}");
}
