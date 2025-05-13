// Here I will be working with Chapter 3

//---------------

// Variables and Mutability

// This is just a little snipped that throws an error as the code is immutable by default. Interesting

// fn main() {
//     let x = 5;
//     println!("The value of x is {x}");
//     x = 6;
//     println!("The value of x is {x}");
// }  
 
 // Next I will play with muting the variable

 fn main() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 18;
    println!("The value of x is {x}");
 }

