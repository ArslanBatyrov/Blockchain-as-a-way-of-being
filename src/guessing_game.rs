use std::io;
use std::cmp::Ordering;

use rand::Rng;

// This is a code of a guessing game
fn main() {
    println!("Guess the number! Let me give you a hint: between 1-100 (Inclusive)");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // Mutability is amazing

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, //For the sake of making the game less fragile in term os a crash due to the non-numerical input
        };


        println!("You guessed: {guess}"); 

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small machine, try again!"),
            Ordering::Greater => println!("Too big, try again!"),
            Ordering::Equal => {
                println!("You win! Well Done!");
                break;  //Done to stop the game after a correct number is guessed
            }
        }
}

}