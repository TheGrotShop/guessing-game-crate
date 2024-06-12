//! # Guessing Game
//!
//! A simple number guessing game in Rust. This crate allows you to play a number guessing game
//! where you have to guess a randomly generated number between 1 and 100.

use rand::Rng;
use std::cmp::Ordering;
use std::io;

/// Starts the guessing game.
///
/// The game generates a random number between 1 and 100, and the player is prompted to guess the number.
/// The player receives feedback whether their guess is too small, too big, or correct. The game continues
/// until the player guesses the correct number.
pub fn play() {
    println!("Welcome to the Guessing Game!");
    println!("Guess a number between 1 and 100");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

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

fn main() {
    play();
}
