//Here I will be revising the enetire week 1

// tuples Revision:
// fn main (){
//     let pair: (i32, &str) = (100, "Rust");
//     let (num, text) = pair;
//     println!("Number: {}", pair.0);
//     println!("Text: {}", pair.1);
// }



// Next: Structs Revision

// struct Man {
//     height: u64,
//     name: String,
//     age: u64,
// }


// impl Man {
//     fn birthday(&mut self){
//       self.age += 1;
//       println!("Happy {}th birthday, {}!", self.age, self.name);  
//     }
// }

// fn main() {
//     let mut arslan = Man { name: "Arslan".into(), age: 20, height: 188,};
//     arslan.birthday();
// }


// Next: Vectors: 
// Vectors can only store the vlaue of the similar type



// fn main() {
//     let mut v = vec![1, 2, 3];
//     v.push(19);

//     let forth = &v[3];
//    println!("{forth}");
// }


// // revising range 
// for n in 1..101{ 

// }  // inclusive start, exlusive end

// for n in 1..=101{

// } // enclusive start and end

// Next:  revising Enums

// enum Fruit {
//     Apple,
//     Orange,
//     Grape,
// }

// Fruit::Orange

// Next: Revising Options

// opiton is an enum that we defined in a SL

//Next: revising Matching

// Revising memory management in Rust
// mainly everyhting is stores on stack, if you store something on stack
// the thing must have a fixed size, copying is cheap

// For more complex items with,for example variabel size, we store them on the heap
// We try to avoid copying from the heap

// Revising Ownership:
// variable owns a value

// let mut my_vec = vec![1, 2, 3];


//Next: covering references

// Reference must not outlive the referrent
// There are 2 types of references mutable ans shared
// using refernece to a value is called borrowing!


// you can read hsared references but cannot mutate them
//you can have as many shared referneces as you want

// a mutable reference, mean you can read and modify the referent
// if you have a mut ref to a value, you cannot have any other ref. Just one
// This is called 'Mulitple readers or single writer' principle


// Strings as Data types
// Sting is stored on a heap, we can append literals to it
// fn main(){
// let mut title = String::from("Arslan's Rust Course");
// title.push_str(" Will Go Well");
// println!("{}", title); 
// }


// Revise: Traits. 

// pub trait Summary{
//         fn sumarise(&self) -> String;
//     }

//     pub struct  Tweet {
//         pub username: String,
//         pub content: String,
//         pub reply: bool,
//         pub retweet: bool, 
//     }

//     impl Summary for Tweet {
//         fn summarize(&self) -> String {
//             format!("{}: {}", self.username, self.content)
//         }
//     }

//     fn main() {
//     let tweet = Tweet {
//         username: String::from("arslan"),
//         content: String::from("Learning Rust is awesome!"),
//         reply: false,
//         retweet: false,
//     };

//     println!("{}", tweet.summarize());
// }


// revising traits by example
// trait Greet {
//     fn greet(&self) -> String;
// }


// struct Car {
//     model: String,
// }



// trait Describe {
//     fn describe(&self) -> String;
// }

// impl Describe for Car {
//     fn describe(&self) -> String {
//         format!("This car is a {}", self.model)
//     }
// }

// fn main(){
//     let my_car = Car {model: String::from("Tesla Model S")};
//     println!("{}", my_car.describe());
// }

// next: Introduciton to vectors
// the most used type of collections.

// let v: Vec<i32> = Vec::new(); //cretead a new vector without using a macro

fn main(){
    let mut v: Vec<i32> = Vec::new();
    v.extend([2, 3, 7, 9, 23, 65]);

    println!("{:?}",v)
}


