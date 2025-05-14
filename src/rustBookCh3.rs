// Here I will be working with Chapter 3

//---------------

// 3.1 Variables and Mutability

// This is just a little snipped that throws an error as the code is immutable by default. Interesting

// fn main() {
//     let x = 5;
//     println!("The value of x is {x}");
//     x = 6;
//     println!("The value of x is {x}");
// }  
 
 // Next I made a variable mutable:

//  fn main() {
//     // const FORTY_FOR_TIMES_FOUR: u32 = 44 * 4;
//     let mut x = 5;
//     println!("The value of x is {x}");
//     x = 18;
//     println!("The value of x is {x}");
//     // Next lines of code are th epractice of shadowing. 
//     // This is useful when you want to change the type of the data stores(if required)
//     let m = 12;
//     let m = m -9;
//     {
//         let m = m * 2;
//         println!("The value of m in the inner scope is: {m}");
//     }
//  }

// 3.2 Next I am covering Data Types


// --- There are 2 different types in Rust: Scalar and Compound

// Scalar types represent a single value: numbers, boolean, integers and
// floating point numbers
// INTEGERS: signed can be negative(i8,16,32 etc). 

// Signed integers are being stores in Rust by using a 2 complement method : Negative numbers flip 0 - 1 and vice versa then add 1. 
fn main() {
    let guess: u32 ="42".parse().expect("Not a number!");
}



