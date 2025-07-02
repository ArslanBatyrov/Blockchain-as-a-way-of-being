// Here i will be revising the entire week 2
//Exercise from lesson 5: 

// functions1.rs
// Make me compile! Execute `rustlings hint functions1` for hints :)

// I AM NOT DONE
// fn call_me() {
//     println!("Call me!");
// }

// fn main() {
//     call_me();
// }


// functions2.rs
// Make me compile! Execute `rustlings hint functions2` for hints :)

// I AM NOT DONE

// fn main() {
//     call_this(3);
// }

// fn call_this(num:i32) {
//     for i in 0..num {
//         println!("Loop! number {}", i + 1);
//     }
// }


// Next: functions3.rs

// functions3.rs
// Make me compile! Execute `rustlings hint functions3` for hints :)

// I AM NOT DONE

// fn main() {
//     call_this(5);
// }

// fn call_this(num: u32) {
//     for i in 0..num {
//         println!("Loop now {}", i + 1);
//     }
// }  //you just need to put a parameter into the funciton if you want it to compile(u32)


// NEXT: functions4.rs
// Make me compile! Execute `rustlings hint functions4` for hints :)

// This store is having a sale where if the price is an even number, you get
// 10 Rustbucks off, but if it's an odd number, it's 3 Rustbucks off.

// I AM NOT DONE

// fn main() {
//     let original_price = 51;
//     println!("Your sale price is {}", sale_price(original_price));
// }

// fn sale_price(price: i32) -> i32 {
//     if is_even(price) {
//         price - 10
//     } else {
//         price - 3
//     }
// }

// fn is_even(num: i32) -> bool {
//     num % 2 == 0
// }  //after the arrow -> you must specify the type you want to return

//NEXT:  functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)

// I AM NOT DONE

// fn main() {
//     let answer = square(3);
//     println!("The answer is {}", answer);
// }

// fn square(num: i32) -> i32 {
//     num * num  // we must remove a semicolon to turn the num num inot an expression
// }


//Next: Ifs.

// this one is the exercise If1.rs

// if1.rs

// I AM NOT DONE

// pub fn bigger(a: i32, b: i32) -> i32 {

//     if a > b {
//         a
//     } else {
//             b
//         }
// }
  
// fn main() {}


// if2.rs

// Step 1: Make me compile!
// Step 2: Get the bar_for_fuzz and default_to_baz tests passing!
// Execute the command `rustlings hint if2` if you want a hint :)

// I AM NOT DONE

// fn main() {}

// pub fn fizz_if_foo(fizzish: &str) -> &str {
//     if fizzish == "fizz" {
//         "foo"
//     } 
//     else {
//         "1"
//     }
// }

// // No test changes needed!
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn foo_for_fizz() {
//         assert_eq!(fizz_if_foo("fizz"), "foo")
//     }

//     #[test]
//     fn bar_for_fuzz() {
//         assert_eq!(fizz_if_foo("fuzz"), "bar")
//     }

//     #[test]
//     fn default_to_baz() {
//         assert_eq!(fizz_if_foo("literally anything"), "baz")
//     }
// }

// primitive_types1.rs
// Fill in the rest of the line that has code missing!
// No hints, there's no tricks, just get used to typing these :)

// I AM NOT DONE

// fn main() {
//     // Booleans (`bool`)

//     let is_morning = true;
//     if is_morning {
//         println!("Good morning!");
//     }

//     let is_evening = false;// Finish the rest of this line like the example! Or make it be false!
//     if is_evening {
//         println!("Good evening!");
//     }
// }


// primitive_types2.rs
// Fill in the rest of the line that has code missing!
// No hints, there's no tricks, just get used to typing these :)

// I AM NOT DONE

// fn main() {
//     // Characters (`char`)

//     // Note the _single_ quotes, these are different from the double quotes
//     // you've been seeing around.
//     let my_first_initial = 'C';
//     if my_first_initial.is_alphabetic() {
//         println!("Alphabetical!");
//     } else if my_first_initial.is_numeric() {
//         println!("Numerical!");
//     } else {
//         println!("Neither alphabetic nor numeric!");
//     }

//     let your_character = '7'; 
//    if your_character.is_alphabetic() {
//         println!("Alphabetical!");
//     } else if your_character.is_numeric() {
//         println!("Numerical!");
//     } else {
//         println!("Neither alphabetic nor numeric!");
//     }
// }


// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` for hints!

// I AM NOT DONE

// fn main() {
//     let a = [1,2,3,4,5,6,7,8,9,10,11100];

//     if a.len() >= 100 {
//         println!("Wow, that's a big array!");
//     } else {
//         println!("Meh, I eat arrays like that for breakfast.");
//     }
// }

// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` for hints!!

// I AM NOT DONE

// #[test]
// fn slice_out_of_array() {
//     let a = [1, 2, 3, 4, 5];

//     let nice_slice = &a[1..4];

//     assert_eq!([2, 3, 4], nice_slice)
// }

// fn main(){}


// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` for hints!

// I AM NOT DONE

// fn main() {
//     let cat = ("Furry McFurson", 3.5);
//     let (name,age) = cat;

//     println!("{} is {} years old.", name, age);
// }

// primitive_types6.rs
// Use a tuple index to access the second element of `numbers`.
// You can put the expression for the second element where ??? is so that the test passes.
// Execute `rustlings hint primitive_types6` for hints!

// I AM NOT DONE

// #[test]
// fn indexing_tuple() {
//     let numbers = (1, 2, 3);
//     // Replace below ??? with the tuple indexing syntax.
//     let second = numbers.1;

//     assert_eq!(2, second,
//         "This is not the 2nd number in the tuple!")
// }

// fn main(){}


// iterators1.rs
//
//  Make me compile by filling in the `???`s
//
// When performing operations on elements within a collection, iterators are essential.
// This module helps you get familiar with the structure of using an iterator and
// how to go through elements within an iterable collection.
//
// Execute `rustlings hint iterators1` for hints :D

// I AM NOT DONE

// fn main () {
//     let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

//     let mut my_iterable_fav_fruits = my_fav_fruits.iter();   // TODO: Step 1

//     assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
//     assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));     // TODO: Step 2
//     assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
//     assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach"));     // TODO: Step 3
//     assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
//     assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));     // TODO: Step 4
// }

// strings1.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings1` for hints ;)

// I AM NOT DONE

// fn main() {
//     let answer = current_favorite_course();
//     println!("My course is {}", answer);
// }

// fn current_favorite_course() -> String {
//      String::from("Solana")
// }


// strings2.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings2` for hints :)

// I AM NOT DONE

// fn main() {
//     let word = String::from("green"); // Try not changing this line :)
//     if is_a_color_word(&word) {
//         println!("That is a color word I know!");
//     } else {
//         println!("That is not a color word I know.");
//     }
// }

// fn is_a_color_word(attempt: &str) -> bool {
//     attempt == "green" || attempt == "blue" || attempt == "red"
// }

// variables1.rs
// Make me compile!
// Execute the command `rustlings hint variables1` if you want a hint :)

// I AM NOT DONE

// fn main() {
//     let y = 5;
//     println!("y has the value {}", y);
// }


// variables2.rs
// Make me compile! Execute the command `rustlings hint variables2` if you want a hint :)

// I AM NOT DONE

// fn main() {
//     let x = 5;
//     if x == 10 {
//         println!("x is ten!");
//     } else {
//         println!("x is not ten!");
//     }
// }

// variables3.rs
// Make me compile! Execute the command `rustlings hint variables3` if you want a hint :)

// I AM NOT DONE

// fn main() {
//     let x: i32 = 5;
//     println!("Number {}", x);
// }

// variables4.rs
// Make me compile! Execute the command `rustlings hint variables4` if you want a hint :)

// I AM NOT DONE

// fn main() {
//     let mut x = 3;
//     println!("Number {}", x);
//     x = 5; // don't change this line
//     println!("Number {}", x);
// }


// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)

// I AM NOT DONE

// fn main() {
//     let number = "T-H-R-E-E"; // don't change this line
//     println!("Spell a Number : {}", number);
//     let number = 3; // don't rename this variable
//     println!("Number plus two is : {}", number + 2);
// }


// variables6.rs
// Make me compile! Execute the command `rustlings hint variables6` if you want a hint :)

// I AM NOT DONE

// const NUMBER: u32 = 3;
// fn main() {
//     println!("Number {}", NUMBER);
// }


// Next: Lesson 6, week 1, Collections Rustlings

// hashmap1.rs
// A basket of fruits in the form of a hash map needs to be defined.
// The key represents the name of the fruit and the value represents
// how many of that particular fruit is in the basket. You have to put
// at least three different types of fruits (e.g apple, banana, mango)
// in the basket and the total count of all the fruits should be at
// least five.
//
// Make me compile and pass the tests!
//
// Execute the command `rustlings hint hashmap1` if you need
// hints.

// I AM NOT DONE

// use std::collections::HashMap;

// fn fruit_basket() -> HashMap<String, u32> {
//     let mut basket = HashMap::new(); // TODO: declare your hash map here.

//     // Two bananas are already given for you :)
//     basket.insert(String::from("banana"), 2);
//     basket.insert(String::from("apple"), 4);
//     basket.insert(String::from("banana"), 7);

//     // TODO: Put more fruits in your basket here.

//     basket
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn at_least_three_types_of_fruits() {
//         let basket = fruit_basket();
//         assert!(basket.len() >= 3);
//     }

//     #[test]
//     fn at_least_five_fruits() {
//         let basket = fruit_basket();
//         assert!(basket.values().sum::<u32>() >= 5);
//     }
// }

// fn main(){}


// hashmap2.rs

// A basket of fruits in the form of a hash map is given. The key
// represents the name of the fruit and the value represents how many
// of that particular fruit is in the basket. You have to put *MORE
// THAN 11* fruits in the basket. Three types of fruits - Apple (4),
// Mango (2) and Lychee (5) are already given in the basket. You are
// not allowed to insert any more of these fruits!
//
// Make me pass the tests!
//
// Execute the command `rustlings hint hashmap2` if you need
// hints.

// I AM NOT DONE

// use std::collections::HashMap;

// #[derive(Hash, PartialEq, Eq)]
// enum Fruit {
//     Apple,
//     Banana,
//     Mango,
//     Lychee,
//     Pineapple,
// }

// fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {
//     let fruit_kinds = vec![
//         Fruit::Apple,
//         Fruit::Banana,
//         Fruit::Mango,
//         Fruit::Lychee,
//         Fruit::Pineapple,
//     ];

//     for fruit in fruit_kinds {
//         // TODO: Put new fruits if not already present. Note that you
//         // are not allowed to put any type of fruit that's already
//         // present!
//         if !basket.contains_key(&fruit) {
        
//             match fruit {
//                 Fruit::Banana => {basket.insert(Fruit::Banana, 3);}
//                 Fruit::Pineapple => {basket.insert(Fruit::Pineapple, 4);}
//                 _ =>{}
//             }
//         }
        
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     fn get_fruit_basket() -> HashMap<Fruit, u32> {
//         let mut basket = HashMap::<Fruit, u32>::new();
//         basket.insert(Fruit::Apple, 4);
//         basket.insert(Fruit::Mango, 2);
//         basket.insert(Fruit::Lychee, 5);

//         basket
//     }

//     #[test]
//     fn test_given_fruits_are_not_modified() {
//         let mut basket = get_fruit_basket();
//         fruit_basket(&mut basket);
//         assert_eq!(*basket.get(&Fruit::Apple).unwrap(), 4);
//         assert_eq!(*basket.get(&Fruit::Mango).unwrap(), 2);
//         assert_eq!(*basket.get(&Fruit::Lychee).unwrap(), 5);
//     }

//     #[test]
//     fn at_least_five_types_of_fruits() {
//         let mut basket = get_fruit_basket();
//         fruit_basket(&mut basket);
//         let count_fruit_kinds = basket.len();
//         assert!(count_fruit_kinds >= 5);
//     }

//     #[test]
//     fn greater_than_eleven_fruits() {
//         let mut basket = get_fruit_basket();
//         fruit_basket(&mut basket);
//         let count = basket.values().sum::<u32>();
//         assert!(count > 11);
//     }
// }


// fn main(){}

// vec1.rs
// Your task is to create a `Vec` which holds the exact same elements
// as in the array `a`.
// Make me compile and pass the test!
// Execute the command `rustlings hint vec1` if you need hints.

// I AM NOT DONE

// fn array_and_vec() -> ([i32; 4], Vec<i32>) {
//     let a = [10, 20, 30, 40]; // a plain array
//     let v = vec![10,20,30,40];// TODO: declare your vector here with the macro for vectors

//     (a, v)
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_array_and_vec_similarity() {
//         let (a, v) = array_and_vec();
//         assert_eq!(a, v[..]);
//     }
// }

// vec2.rs
// A Vec of even numbers is given. Your task is to complete the loop
// so that each number in the Vec is multiplied by 2.
//
// Make me pass the test!
//
// Execute the command `rustlings hint vec2` if you need
// hints.

// I AM NOT DONE

// fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
//     for i in v.iter_mut() {
//         *i = *i * 2; 
//     }

    
//     println!("{:?}", v);

//     v  
// }


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_vec_loop() {
//         let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
//         let ans = vec_loop(v.clone());

//         assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
//     }
// }

// fn main(){}

// Next: enums.1

// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!

// I AM NOT DONE

// #[derive(Debug)]

// enum Message {
//     Quit,
//     Echo,
//     Move,
//     ChangeColor,
// }

// fn main() {
//     println!("{:?}", Message::Quit);
//     println!("{:?}", Message::Echo);
//     println!("{:?}", Message::Move);
//     println!("{:?}", Message::ChangeColor);
// }

// enums2.rs
// Make me compile! Execute `rustlings hint enums2` for hints!

// I AM NOT DONE

// #[derive(Debug)]
// enum Message {
//     // TODO: define the different variants used below
//     Move{x: i32, y: i32},
//     Echo(String),
//     ChangeColor(i32, i32, i32),
//     Quit,
// }

// impl Message {
//     fn call(&self) {
//         println!("{:?}", &self);
//     }
// }

// fn main() {
//     let messages = [
//         Message::Move { x: 10, y: 30 },
//         Message::Echo(String::from("hello world")),
//         Message::ChangeColor(200, 255, 255),
//         Message::Quit,
//     ];

//     for message in &messages {
//         message.call();
//     }
// }

// enums3.rs
// Address all the TODOs to make the tests pass!

// I AM NOT DONE

// use std::fmt::DebugStruct;

// enum Message {
//     // TODO: implement the message variant types based on their usage below
//     ChangeColor((u8, u8, u8)),
//     Echo(String),
//     Move(Point),
//     Quit,
// }

// struct Point {
//     x: u8,
//     y: u8,
// }

// struct State {
//     color: (u8, u8, u8),
//     position: Point,
//     quit: bool,
// }

// impl State {
//     fn change_color(&mut self, color: (u8, u8, u8)) {
//         self.color = color;
//     }

//     fn quit(&mut self) {
//         self.quit = true;
//     }

//     fn echo(&self, s: String) {
//         println!("{}", s);
//     }

//     fn move_position(&mut self, p: Point) {
//         self.position = p;
//     }

//     fn process(&mut self, message: Message) {
//         match message {
//             Message::ChangeColor(color) => self.change_color(color),
//             Message::Echo(text) => self.echo(text),
//             Message::Move(point) => self.move_position(point),
//             Message::Quit => self.quit(),

//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_match_message_call() {
//         let mut state = State {
//             quit: false,
//             position: Point { x: 0, y: 0 },
//             color: (0, 0, 0),
//         };
//         state.process(Message::ChangeColor((255, 0, 255)));
//         state.process(Message::Echo(String::from("hello world")));
//         state.process(Message::Move(Point { x: 10, y: 15 }));
//         state.process(Message::Quit);

//         assert_eq!(state.color, (255, 0, 255));
//         assert_eq!(state.position.x, 10);
//         assert_eq!(state.position.y, 15);
//         assert_eq!(state.quit, true);
//     }
// }

// fn main() {

// }

// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

// Execute `rustlings hint generics1` for hints!

// I AM NOT DONE

// fn main() {
//     let mut shopping_list: Vec<&str> = Vec::new();
//     shopping_list.push("milk");
// }

// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.

// Execute `rustlings hint generics2` for hints!

// I AM NOT DONE


// struct Wrapper {
//     value: u32,
// }

// impl Wrapper {
//     pub fn new(value: u32) -> Self {
//         Wrapper { value }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn store_u32_in_wrapper() {
//         assert_eq!(Wrapper::new(42).value, 42);
//     }

//     #[test]
//     fn store_str_in_wrapper() {
//         assert_eq!(Wrapper::new("Foo").value, "Foo");
//     }
// }

// extra revision of generics is required for me to solve the above exercise
// Next will be the revision to the solve the above exercise.


// Here I remove the duplication issues using a function
// fn main(){
//     let number_list =  vec![34, 50, 25, 100, 65];
//     largest(&number_list);
    // let mut largest = number_list[0];

    // for number in &number_list {
    //     if *number > largest {
    //         largest = *number;
    //     } 
    // }
    //
    // println!("The largest number is {}", largest);
//}

// fn largest(list: &[i32])-> &i32{
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest{
//             largest = item;
//         }
        
//     }
//     println!("{}",largest);
//     largest
// }


// removing a duplication issue using generic data types.

// Using generics with Functions:

// fn largest<T>(list: &[T]) -> &T {

// }

// Using generics in Structs

// struct Point<T>{
//     x: T,
//     y: T,
// }

// struct Point<T, K>{
//     x: T,
//     y: K,
// }

// fn main(){
//     let integer = Point {x: 5, y: 'o'};
//     let float = Point{x: 5.0, y:10.0};
// }

// Next: Using generics in Enums

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// Next: Definitions of generic in Methods

// Point<T> struct with method

struct Point<T>{
    x: T,
    y: T,
}

impl<T> Point<T> {
    
    fn x(&self) -> &T{
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

fn main(){
    let p = Point {x:5, y:10};

    println!("p.y = {}", p.y());
    println!("p.x = {}", p.x());
}