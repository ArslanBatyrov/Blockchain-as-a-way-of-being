use std::io;

fn main() {
    println!("Guess the number! Let me give you a hint...");

    let x = 7;
    let y = 2;
    println!("x = {x} and y + 2 = {}", y + 2);


    println!("Please input your guess.");

    let mut guess = String::new(); // Mutability is amazing

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}"); 
}