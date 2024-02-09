use rand::{self, Rng};
use std::io::Write;

pub fn main() {
    let mut buf = String::with_capacity(8);
    let rand_num = rand::thread_rng().gen_range(1..=100);

    loop {
        print!("\nGuess a number from 1 to 100: ");
        std::io::stdout().flush().unwrap();
        match std::io::stdin().read_line(&mut buf) {
            Ok(_) => {
                match buf.trim().parse::<u8>() {
                    Ok(val) if (1..=100).contains(&val)  => {
                        match val.cmp(&rand_num) {
                            std::cmp::Ordering::Equal => {
                                println!("Correct!");
                                break;
                            }
                            std::cmp::Ordering::Less => println!("Number to small"),
                            std::cmp::Ordering::Greater => println!("Number to large"),
                        }
                    }
                    _ => println!("Must enter a valid integer between 1 and 100")
                }
            },
            Err(_) => println!("Bad input, try again"),
        }
        buf.clear()
    }
}