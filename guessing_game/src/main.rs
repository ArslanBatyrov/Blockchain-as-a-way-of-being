use std::io;
use std::cmp::Ordering;

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

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}"); 

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small machine, try again!"),
        Ordering::Greater => println!("Too big, try again!"),
        Ordering::Equal => println!("You win! Well Done!"),
    }

   // println!("secret number is {secret_number}");
}