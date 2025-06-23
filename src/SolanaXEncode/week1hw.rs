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

// fn main(){
//     let mut v: Vec<i32> = Vec::new();
//     v.extend([2, 3, 7, 9, 23, 65]);

//     println!("{:?}",v)
// }

// Next: Covering iterators

// fn main() {
//         let v1 = vec![4, 7, 9];
//         let mut v1_iter = v1.iter();

//         assert_eq!(v1_iter.next(), Some(&4));
//         assert_eq!(v1_iter.next(), Some(&7));
//         assert_eq!(v1_iter.next(), Some(&9));
//         assert_eq!(v1_iter.next(), None);
//     }

// fn main(){
//     let numbers_into_bool = vec![1,2,3,4];
//     let pure_bool: Vec<bool> = numbers_into_bool
//     .iter()
//     .map(|x|x%2 != 0)
//     .collect();
//     println!("{:?}", pure_bool);

// }

// Next: Lesson 12. NFTs

// What is an inscription

// means to add the digital assets on the blockchain
// the way to add asset data on chain




// Next: Shadowing is the way to change the value of the immutable variables

// fn main() {
//     let y = 2;
//     let y = y * 9;

//     {
//         let y = y * 7;
//         println!("Here y is: {y}");
//     }

//     println!("Here y is: {y}");
// }

// Next: Lifetime constraints on references
// fn main(){
// let a = 2;
// {
//     let x = &a;
//     assert_eq!(*x, 4)
// }
// }
 
// Next: Slices


// use std::mem;

// // This function borrows a slice
// fn analyze_slice(slice: &[i32]) {
//     println!("first element of the slice: {}", slice[0]);
//     println!("the slice has {} elements", slice.len());
// }

// fn main() {
//     // Fixed-size array (type signature is superfluous)
//     let xs: [i32; 5] = [1, 2, 3, 4, 5];

//     // All elements can be initialized to the same value
//     let ys: [i32; 500] = [0; 500];

//     // Indexing starts at 0
//     println!("first element of the array: {}", xs[0]);
//     println!("second element of the array: {}", xs[1]);

//     // `len` returns the count of elements in the array
//     println!("number of elements in array: {}", xs.len());

//     // Arrays are stack allocated
//     println!("array occupies {} bytes", mem::size_of_val(&xs));

//     // Arrays can be automatically borrowed as slices
//     println!("borrow the whole array as a slice");
//     analyze_slice(&xs);

//     // Slices can point to a section of an array
//     // They are of the form [starting_index..ending_index]
//     // starting_index is the first position in the slice
//     // ending_index is one more than the last position in the slice
//     println!("borrow a section of the array as a slice");
//     analyze_slice(&ys[1 .. 4]);

//     // Out of bound indexing causes compile error
//     //println!("{}", xs[5]);
// }


// Next:  Error Handling
// There is a difference between recoverable and unrecoverable errors.
// rust distinguishes between the two types of errors.
// Recoverable errors are handled using the Result type.

// enum Result<T, E>
// {
//     Ok(T), Err(E),
// }

// // or we can use matching

// let f = File::open("foo.txt");
// let f = match f {
//     Ok(file) => file,
//     Err(error) => // Whatever error handling you have got
// }

// // we can use ? shortcut
// let mut f = File::open("foo.txt")?;

// // if the error is unrecoverable then we use a panic macro as it means that there
// // is bug or weakness is found in the code

// panic!("Your message about the bug");


// Next: Traits and you video, from Exercise 2, week 1


fn main(){
    struct Dwarf {
        name: String,
    }

    struct Elf {
        name: String,
    }

    struct HalfOrc {
        name: String,
    }

     struct Human {
        name: String,
    }


    let turkmen_dwarf = Dwarf{
        name: String::from("JennetDwarf"),
    };

    let turkmen_halforc = HalfOrc{
        name: String::from("SoltanHalfOrc")
    };

     let turkmen_elf = HalfOrc{
        name: String::from("ArslanElf")
    };

     let turkmen = HalfOrc{
        name: String::from("Sona")
    };

    

    pub trait Constitution{
        fn constitution_bonus(&self) -> u8{
            0
        }
    }

    impl Constitution for Dwarf {
        fn constitution_bonus(&self) -> u8 {
            2
        }
    }

     impl Constitution for HalfOrc {
        fn constitution_bonus(&self) -> u8 {
            1
        }
    }

   
    turkmen.constitution_bonus();
    turkmen_elf.constitution_bonus();
    turkmen_halforc.constitution_bonus(); //returns 1
    turkmen_dwarf.constitution_bonus(); // returns 2
}
