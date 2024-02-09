use std::{fmt::Write, collections::HashMap, fs};

pub fn main() {
    let file_str = match std::env::args().nth(1) {
        Some(file_path) => {
            match fs::read_to_string(file_path) {
                Ok(f) => f,
                Err(err) => panic!("Unable to open file: {}", err)
            }
        }
        None => {
            panic!("Must provide file path as argument");
        }
    };
    let mut unique_words: HashMap<&str, u64> = HashMap::with_capacity(256);

    let mut most_common_words: Vec<&str> = vec![];
    let mut most_common_word_cnt: u64 = 0;
    for word in file_str.split_whitespace() {
        let f = unique_words.entry(word).and_modify(|s| *s += 1).or_insert(1);
        match (*f).cmp(&most_common_word_cnt) {
            std::cmp::Ordering::Greater => {
                most_common_words.clear();
                most_common_words.push(word);
                most_common_word_cnt = *f;
            },
            std::cmp::Ordering::Equal => {
                most_common_words.push(word);
            }
            _ => (),
        }
    }

    match fs::write(
        "words.txt",
        unique_words.iter().fold(
            String::with_capacity(unique_words.capacity() * 34),
            |mut o, s| {
                let _ = writeln!(o, "Word: '{}' occurred {} time(s)", s.0, s.1);
                o
            }
        )
    ) {
        Ok(_) => (),
        Err(_) => panic!("Failed to write result to file")
    }

    println!(
        "The most common word(s) that occurred {} and are: {}",most_common_word_cnt,  most_common_words.join(",")
    )
}