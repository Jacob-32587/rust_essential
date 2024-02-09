use std::env;
use std::io::{prelude::*, BufReader};

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Incorrect number of arguments supplied: {}", args.len());
    }

    let file = match std::fs::File::open(&args[1]) {
        Ok(f) => f,
        Err(e) => panic!(
            "Error opening `{}/{}`\n{}",env::current_dir().unwrap_or_default().to_str().unwrap_or_default(),
            args[1],
            e
        )
    };
    let buf_read = BufReader::new(file);

    for (i, l) in buf_read.lines().enumerate() {
        if l.unwrap().trim() == args[2] {
            println!("Found name at line: {}", i + 1)
        }
    }
}