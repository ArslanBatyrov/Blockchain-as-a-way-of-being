// Zero knowldge proofs are usually based on Rust



// Structs: it is a unique data type you create

struct Person: {
    name: String,
    age: u32,
    email: String,

}  //You create new data type, so that in the future 
   //you can use it when required in the future

   fn main(){
    let mut alice = Person {name: "Arslan", age:20};

   } // the way to initialise struct


   //Vectors: Vectors give you moore flexibility that arrays
// if you see exclamation mark = it is macro
let names = vec!["Holy", "Alissa", "Arslan"];


// Next Arrays:

// Arrays work in the same way as in another languages. 
// Fature of arrays in rust is that they are static. Once created you cannot change the length
// Can't add or remove items

let variable_name = [value1, value2, value3];

//You can't change the length but you can change the items withign the array


// Next: What is a slice?

//give flexibility os length is not specified at compile time

// use 'as' keyword to change one data type into the other explicitly

let a = 12;
let b = a as string; 

// What are Enums?

enum Fruit {
    Apple,
    Orange,
    Grape
}

//Referencing the enum :

Fruit::Orange;

//Functions in Rust

fn my_func(a: u32) -> bool {
    if a ==0 {
        return false;
    }
    a == 7 // This piece will return whether a is 7 or not
    // we already specified boolean return with ->
}


// Control flow

// if is an expression not a statement. SO it will always return value

fn main(){
    let condition = true;
    let number = if condition {5} else {6}; //here if literally retuns 5 as 
    //it is an expression

    println!("The value pf number is {:?}", number);
}

// Next: what is an option

// options make code well designed. It is an enum defined within a standard library


// New way of coding for Rust --> Matching

//Example: Matching an enum

enum Coin {
    Penny,
    Nickel,
    Dime,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny =>1,
        Coin::Nickel =>5,
        Coin::Dime =>10,
    }
} //function is apparently an expression too

// Example Matching against an option Enum.

fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32>{
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
} // Here we matched option enum. 


//----------------------------------------------------------

// Day 3: Socialisation with teammates + Rust

// Rustlings - set of exercises to learn Rust
// Rust by example: LEarning rust by imitatin



// Rust impose rules that you need to follow at a compile time. No the run time



// Next Topic: Ownership

let mut my_vec = vec![1,2,3]; // -> The my_vec owns the vector now

// things to avoid:
// 1. Vector should not go out of scope

// As soon as variable goes out of scope, the value of the variable is dropped:

fn mainn(){
    {
        let a = 2; // This value exists only till
    }// this point

    // here that value is dropped
}


// Moves: If we try to reassign the comples data types, we do not copy, we move

let a = vec![1,2,3];
let b = a;
let c = a: // Mistake is here as a value is empty, non existent
// as value was moved to b



//use  referneces instead of moves

// without references we would be transferring ownership all the time

// referneces allow to interact with the value without owning it
// Rule: reference itself must not outlive the value you are referring to


// There are 2 times of references: Shares and Mutable. 
// Both do not take ownership

//Shared Reference example: Read but not mutate the thing we are referring to
// mutable reference: We modify the thing we are referring to.
// Only 1 mutable reference at 1 time can be used. Shared: Unlimited
// &mut -> Used to declare a mutable reference


//Example of mutable references or borrowing
fn main() {
    let mut s = String::from("Hello");
    let s1 = &mut s;

    s1.push_str("bootcamp"); // s1 does not own the "Hello", it just mutates it


}


// You can dereference with *


// Next: traits
// What are traits? 

// define the beahviour that data types have

pub trait Summary {
    fn summarise(&self) -> String;
} // this is how we define traits




// Day 4(that I missed.) Self Study

// Slide 1: Lifetime constraints on references
// realised that to understand lesson 4 I must go through 1,2,3 better and i depth




