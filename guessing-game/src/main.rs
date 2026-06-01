use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    let secret_num : u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Guess a number");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess_num : u32 = match guess.trim().parse() {
            Err(_) => continue,
            Ok(num) => num,
        };

        match guess_num.cmp(&secret_num) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You won!");
                break;
            },
        }
    }
}
