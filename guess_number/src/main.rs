extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

    // Generate a random integer between 1-100
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess");
        let mut guess = String::new();

        // Read line from command line
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // Exit if input was Q
        if guess.trim().to_lowercase() == "q" {
            println!("Exiting the program");
            break;
        }

        // Parse input to integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // .expect("Please type a number");

        print!("You guessed {}. ", guess);

        let mut result = false;
        // Compare input to secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                result = true;
            },
        };
        
        // Exit if found match
        if result {
            println!("Exiting the program");
            break;
        }
    }
}