use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number! Let me give you a hint...");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let x = 7;
    let y = 2;
    println!("x = {x} and y + 2 = {}", y + 2);

    println!("Please input your guess.");

    let mut guess = String::new(); // Mutability is amazing

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}"); 
    
    println!("secret number is {secret_number}");
}