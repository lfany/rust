extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("the secret number is: {}", secret_number);

    loop {
        println!("Plz input your guess.");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read Line");

        let guess: u32 = match guess.trim().parse() {
          Ok(num) => num,
            Err(_) => continue,
        };
//            .expect("Please type a number!");
        //    guess = String::from(guess.trim());

        println!("Your Guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Less"),
            Ordering::Greater => println!("Greater"),
            Ordering::Equal => {
                println!("Equal");
                break;
            }
        }

        println!("#{}#{}#", guess, secret_number);
    }
}