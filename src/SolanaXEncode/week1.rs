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
    a == 7
}
