use rand::Rng;
use std::{cmp::Ordering, io};
use colored::Colorize;

fn main() {
    loop {
        let mut guess = String::new();
        let secret_number = rand::thread_rng().gen_range(1, 100);

        println!("Enter number between 1 to 100");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small, Just what she said!".red()),
            Ordering::Equal => {
                println!("{}", "Yeah hooo you win".green());
                break;
            }
            Ordering::Greater => println!("{}", "Too big".red()),
        }

        println!("Your guess: {} random number: {}", guess, secret_number);
    }
}
