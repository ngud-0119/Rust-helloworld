use std::io;
use std::io::Error;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess");

    let mut guess: String = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    let greeting = guess.contains("Hello world")
}

use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess");

    let mut guess: String = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    let greeting = guess.contains("Hello world"); // Add a semicolon at the end of this line

    // Optional: Print the value of the greeting variable
    println!("Greeting: {}", greeting);
}