use rand::Rng;
use std::{cmp::Ordering, io}; // Import io library

fn main() {
    println!("Guess a number!");

    println!("");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // Loop
    loop {
        println!("Please input your number");

        // Declare mutable variable guess as string type
        let mut guess = String::new();

        // Read line
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert string to number 
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // Match number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
