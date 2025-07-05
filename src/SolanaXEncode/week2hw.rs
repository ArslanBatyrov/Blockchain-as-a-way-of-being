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

// struct Point<T>{
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
    
//     fn x(&self) -> &T{
//         &self.x
//     }

//     fn y(&self) -> &T {
//         &self.y
//     }
// }

// fn main(){
//     let p = Point {x:5, y:10};

//     println!("p.y = {}", p.y());
//     println!("p.x = {}", p.x());
// }


// pub struct NewsArticle{
//     pub name: String,
//     pub headline: String,
//     pub author: String,
// }

// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// impl Summary for NewsArticle{
//     fn summarize(&self) -> String {
//         println!("name: {} , headline: {} , author: {}", self.name, self.headline, self.author);
//         format!("{}, by {} ({})", self.headline, self.author, self.author)
//     }
// }


// fn main(){
//     let article = NewsArticle{
//         name: String::from("Jennet News"),
//         headline: String::from("Soltan is playing cards again..."),
//         author: String::from("Arslan")
//     };

//     article.summarize();
// }



// Next: Attempt to solve the things I was not able to

// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.

// Execute `rustlings hint generics2` for hints!

// I AM NOT DONE

// struct Wrapper<T> {
//     value: T,
// }

// impl<T> Wrapper<T> {
//     pub fn new(value: T) -> Self {
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

// fn main(){}


// ------------------------------------

// An imaginary magical school has a new report card generation system written in Rust!
// Currently the system only supports creating report cards where the student's grade
// is represented numerically (e.g. 1.0 -> 5.5).
// However, the school also issues alphabetical grades (A+ -> F-) and needs
// to be able to print both types of report card!

// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to "A+"
// to show that your changes allow alphabetical grades.

// Execute 'rustlings hint generics3' for hints!

// I AM NOT DONE


//----------------------------

// NEXT: Move Semantics

// move_semantics1.rs
// Make me compile! Execute `rustlings hint move_semantics1` for hints :)

// I AM NOT DONE

// fn main() {
//     let vec0 = Vec::new();

//     let mut vec1 = fill_vec(vec0);

//     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

//     vec1.push(88);

//     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
// }

// fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//     let mut vec = vec;

//     vec.push(22);
//     vec.push(44);
//     vec.push(66);

//     vec
// }

// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` for hints :)

// I AM NOT DONE

// fn main() {
//     let vec0 = Vec::new();

//     let mut vec1 = fill_vec(&vec0);

//     // Do not change the following line!
//     println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

//     vec1.push(88);

//     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
// }

// fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
//     let mut vec = vec.clone();

//     vec.push(22);
//     vec.push(44);
//     vec.push(66);

//     vec
// }

// move_semantics3.rs
// Make me compile without adding new lines-- just changing existing lines!
// (no lines with multiple semicolons necessary!)
// Execute `rustlings hint move_semantics3` for hints :)

// I AM NOT DONE

// fn main() {
//     let vec0 = Vec::new();

//     let mut vec1 = fill_vec(vec0);

//     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

//     vec1.push(88);

//     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
// }

// fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
//     vec.push(22);
//     vec.push(44);
//     vec.push(66);

//     vec
// }

// move_semantics4.rs
// Refactor this code so that instead of having `vec0` and creating the vector
// in `fn main`, we create it within `fn fill_vec` and transfer the
// freshly created vector from fill_vec to its caller.
// Execute `rustlings hint move_semantics4` for hints!

// I AM NOT DONE

// fn main() {
//     //let vec0 = Vec::new();

//     let mut vec1 = fill_vec();

//     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

//     vec1.push(88);

//     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
// }

// // `fill_vec()` no longer takes `vec: Vec<i32>` as argument
// fn fill_vec() -> Vec<i32> {
//     let mut vec = Vec::new();
//     // let mut vec = vec;

//     vec.push(22);
//     vec.push(44);
//     vec.push(66);

//     vec
// }

// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` for hints :)

// I AM NOT DONE

// fn main() {
//     let mut x = 100;
//     let y = &mut x;
//     *y += 100;
//     let z = &mut x;
//     *z += 1000;
//     assert_eq!(x, 1200);
// }


// move_semantics6.rs
// Make me compile! `rustlings hint move_semantics6` for hints
// You can't change anything except adding or removing references

// I AM NOT DONE

// fn main() {
//     let data = "Rust is great!".to_string();

//     get_char(&data);

//     string_uppercase(data);
// }

// // Should not take ownership
// fn get_char(data: &String) -> char {
//     data.chars().last().unwrap()
// }

// // Should take ownership
// fn string_uppercase(mut data: String) {
//     data = data.to_uppercase();

//     println!("{}", data);
// }


//---------------------------------------

//Next: Options 

// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints

// I AM NOT DONE

// you can modify anything EXCEPT for this function's signature
// fn print_number(maybe_number: Option<u16>) {
//     println!("printing: {}", maybe_number.unwrap());
// }

// fn main() {
//     print_number(Some(13));
//     print_number(Some(99));

//     let mut numbers: [Option<u16>; 5] = [None; 5];
//     for iter in 0..5 {
//         let number_to_add: u16 = {
//             ((iter * 1235) + 2) / (4 * 16)
//         };

//         numbers[iter as usize] = Some(number_to_add);
//     }
// }


// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints

// I AM NOT DONE

// fn main() {
//     let optional_word = Some(String::from("rustlings"));
//     // TODO: Make this an if let statement whose value is "Some" type
//     if let Some(word) = optional_word {
//         println!("The word is: {}", word);
//     } else {
//         println!("The optional word doesn't contain anything");
//     }

//     let mut optional_integers_vec: Vec<Option<i8>> = Vec::new();
//     for x in 1..10 {
//         optional_integers_vec.push(Some(x));
//     }

//     // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
//     // You can stack `Option<T>`'s into while let and if let
//     while let Some(Some(integer)) = optional_integers_vec.pop() {
//         println!("current value: {}", integer);
//     }
// }



// option3.rs
// Make me compile! Execute `rustlings hint option3` for hints

// I AM NOT DONE

// struct Point {
//     x: i32,
//     y: i32,
// }

// fn main() {
//     let y: Option<Point> = Some(Point { x: 100, y: 200 });

//     match &y {
//         Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
//         _ => println!("no match"),
//     }
//     y; // Fix without deleting this line.
// }


// Next: Coding Challenge

//Given a vector nums, create a new vector called runningSum where each element at index i is the sum of all elements from the beginning of the vector up to index i.
//Return the running sum of nums.

// fn running_sum(nums: Vec<i32>) -> Vec<i32> { 
//     let mut new_vector:Vec<i32> = Vec::with_capacity(nums.len()); 
//     let mut sum = 0;

//     f or i in nums{
//         sum += i;
//         new_vector.push(sum);
//     }
//     new_vector
// }  
  
// fn main() {  
//     println!("{:?}", running_sum(vec![1, 1, 1, 1, 1]));
// }


//------------
// Next: homework7, last rust homeworks
// Error_handling

// errors1.rs
// This function refuses to generate text to be printed on a nametag if
// you pass it an empty string. It'd be nicer if it explained what the problem
// was, instead of just sometimes returning `None`. Thankfully, Rust has a similar
// construct to `Option` that can be used to express error conditions. Let's use it!
// Execute `rustlings hint errors1` for hints!

// I AM NOT DONE

// pub fn generate_nametag_text(name: String) -> Option<String> {
//     if name.len() > 0 {
//         Some(format!("Hi! My name is {}", name))
//     } else {
//         // Empty names aren't allowed.
//         None
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn generates_nametag_text_for_a_nonempty_name() {
//         assert_eq!(
//             generate_nametag_text("Beyoncé".into()),
//             Ok("Hi! My name is Beyoncé".into())
//         );
//     }

//     #[test]
//     fn explains_why_generating_nametag_text_fails() {
//         assert_eq!(
//             generate_nametag_text("".into()),
//             // Don't change this line
//             Err("`name` was empty; it must be nonempty.".into())
//         );
//     }
// }

// I can't solve this because I do not understand error handling at all. Will go thorugh rust book now

// There are 2 types of errors: recoverable and unrecoverable
// For recoverable we have Reult<T, E> while for unrecoverbale rust has a panic! MAcro
// Unrecoverable error is usually indicates some bug i the code
// panic macro automatically clear the stack, it is quite a bit ow work
// If you do want it not to clear a stack you can use panic = "abort", so taht it shuts down
//Without clearing a stack

// Next: Recoverable errors and how to work with them using Result

// enum Result<T,E>{
//     Ok(T),
//     Err(E),
// }


//----------- Chapter 6

// Structs give us the way to group related data, while enums give us the way to say that the data is one of the options

// Ip addreess can only be 1 of teh 2, to day that:

// #[derive(Debug)]
// enum IpAddrKind {
//     V4(String), //This says that both V4 and V6 will have associated String values
//     V6(String), // More concise that using structs even. Now we said: We have 2 options and each of these options have 1 associated string data
// }

// fn main(){
//     let four = IpAddrKind::V4(String::from("Arslan's Data"));
//     let six = IpAddrKind::V6(String::from("jennet's Data"));


//     fn route_printer(Ip_Kind: IpAddrKind){
//         println!("{:?}", Ip_Kind);
//     } // this function now will take onyl V4 or V6 as its parameter

//     route_printer(six);
// }

// Next: Option Enums. The substitute for null in other languages

// fn main(){

// }
// enum Opton<T>{
//     None,
//     Some(T),
// }

// Next: MAtching of enums

// use serde::de::value;

// fn main (){
//     println!("{}",value_in_cents(Coin::Nickel));
// }
// enum Coin{
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,

// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1, 
//         Coin:: Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// One more feature of rust is that arms of the match must be exhaustive, so cover all teh variations of an enums

// ---------- Next: I have finished Ch6, now will go back to Ch9 

// error handling

// use std::fs::File;

// fn main() {
//     let greeting_file_result = File::open("hello_txt"); //Here error is possible! (return is either a file or Error).

//     let greeting_file = match greeting_file_result{
//         Ok(file) => file,
//         Err(error) => panic!("file cannot be opened due to {error:?}, please review the file"),
//     }; // This error handling
// }


//-------------
// Covered a good chunk of Error_handling Ch9, now will try to return to rustlings

// errors1.rs
// This function refuses to generate text to be printed on a nametag if
// you pass it an empty string. It'd be nicer if it explained what the problem
// was, instead of just sometimes returning `None`. Thankfully, Rust has a similar
// construct to `Option` that can be used to express error conditions. Let's use it!
// Execute `rustlings hint errors1` for hints!

// I AM NOT DONE

// pub fn generate_nametag_text(name: String) -> Result<String, String> {
//     if name.len() > 0 {
//         Ok(format!("Hi! My name is {}", name))
//     } else {
//         Err("`name` was empty; it must be nonempty.".into())
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn generates_nametag_text_for_a_nonempty_name() {
//         assert_eq!(
//             generate_nametag_text("Beyoncé".into()),
//             Ok("Hi! My name is Beyoncé".into())
//         );
//     }

//     #[test]
//     fn explains_why_generating_nametag_text_fails() {
//         assert_eq!(
//             generate_nametag_text("".into()),
//             // Don't change this line
//             Err("`name` was empty; it must be nonempty.".into())
//         );
//     }
// }

// fn main(){}


// errors2.rs
// Say we're writing a game where you can buy items with tokens. All items cost
// 5 tokens, and whenever you purchase items there is a processing fee of 1
// token. A player of the game will type in how many items they want to buy,
// and the `total_cost` function will calculate the total number of tokens.
// Since the player typed in the quantity, though, we get it as a string-- and
// they might have typed anything, not just numbers!

// Right now, this function isn't handling the error case at all (and isn't
// handling the success case properly either). What we want to do is:
// if we call the `parse` function on a string that is not a number, that
// function will return a `ParseIntError`, and in that case, we want to
// immediately return that error from our function and not try to multiply
// and add.

// There are at least two ways to implement this that are both correct-- but
// one is a lot shorter! Execute `rustlings hint errors2` for hints to both ways.

// I AM NOT DONE

// use std::num::ParseIntError;

// pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
//     let processing_fee = 1;
//     let cost_per_item = 5;
//     let qty = item_quantity.parse::<i32>()?;

//     Ok(qty * cost_per_item + processing_fee)
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn item_quantity_is_a_valid_number() {
//         assert_eq!(total_cost("34"), Ok(171));
//     }

//     #[test]
//     fn item_quantity_is_an_invalid_number() {
//         assert_eq!(
//             total_cost("beep boop").unwrap_err().to_string(),
//             "invalid digit found in string"
//         );
//     }
// }


// fn main(){}

// errors3.rs
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
// Execute `rustlings hint errors3` for hints!

// I AM NOT DONE

// use std::num::ParseIntError;

// fn main() -> Result<(), std::num::ParseIntError> {
//     let mut tokens = 100;
//     let pretend_user_input = "8";

//     let cost = total_cost(pretend_user_input)?;

//     if cost > tokens {
//         println!("You can't afford that many!");
//     } else {
//         tokens -= cost;
//         println!("You now have {} tokens.", tokens);
//     }

//      Ok(())
// }

// pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
//     let processing_fee = 1;
//     let cost_per_item = 5;
//     let qty = item_quantity.parse::<i32>()?;

//     Ok(qty * cost_per_item + processing_fee)
// }

// errors4.rs
// Make this test pass! Execute `rustlings hint errors4` for hints :)

// I AM NOT DONE

// #[derive(PartialEq, Debug)]
// struct PositiveNonzeroInteger(u64);

// #[derive(PartialEq, Debug)]
// enum CreationError {
//     Negative,
//     Zero,
// }

// impl PositiveNonzeroInteger {
//     fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
//         match value {
//             v if v < 0 => Err(CreationError::Negative),
//             0 => Err(CreationError::Zero),
//             v => Ok(PositiveNonzeroInteger(v as u64))
//         }
//        // Ok(PositiveNonzeroInteger(value as u64))
//     }
// }

// #[test]
// fn test_creation() {
//     assert!(PositiveNonzeroInteger::new(10).is_ok());
//     assert_eq!(
//         Err(CreationError::Negative),
//         PositiveNonzeroInteger::new(-10)
//     );
//     assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
// }

// fn main() {

// }


// errors5.rs

// This program uses a completed version of the code from errors4.
// It won't compile right now! Why?
// Execute `rustlings hint errors5` for hints!

// I AM NOT DONE

// use std::error;
// use std::fmt;
// use std::num::ParseIntError;

// // TODO: update the return type of `main()` to make this compile.
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let pretend_user_input = "42";
//     let x: i64 = pretend_user_input.parse()?;
//     println!("output={:?}", PositiveNonzeroInteger::new(x)?);
//     Ok(())
// }

// // Don't change anything below this line.

// #[derive(PartialEq, Debug)]
// struct PositiveNonzeroInteger(u64);

// #[derive(PartialEq, Debug)]
// enum CreationError {
//     Negative,
//     Zero,
// }

// impl PositiveNonzeroInteger {
//     fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
//         match value {
//             x if x < 0 => Err(CreationError::Negative),
//             x if x == 0 => Err(CreationError::Zero),
//             x => Ok(PositiveNonzeroInteger(x as u64))
//         }
//     }
// }

// // This is required so that `CreationError` can implement `error::Error`.
// impl fmt::Display for CreationError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let description = match *self {
//             CreationError::Negative => "number is negative",
//             CreationError::Zero => "number is zero",
//         };
//         f.write_str(description)
//     }
// }

// impl error::Error for CreationError {}


// errors6.rs

// Using catch-all error types like `Box<dyn error::Error>` isn't recommended
// for library code, where callers might want to make decisions based on the
// error content, instead of printing it out or propagating it further. Here,
// we define a custom error type to make it possible for callers to decide
// what to do next when our function returns an error.

// Make these tests pass! Execute `rustlings hint errors6` for hints :)

// I AM NOT DONE

// use std::num::ParseIntError;

// // This is a custom error type that we will be using in `parse_pos_nonzero()`.
// #[derive(PartialEq, Debug)]
// enum ParsePosNonzeroError {
//     Creation(CreationError),
//     ParseInt(ParseIntError)
// }

// impl ParsePosNonzeroError {
//     // TODO: add another error conversion function here.
// }

// fn parse_pos_nonzero(s: &str)
//     -> Result<PositiveNonzeroInteger, ParsePosNonzeroError>
// {
//     // TODO: change this to return an appropriate error instead of panicking
//     // when `parse()` returns an error.
//     let x: i64 = s.parse().unwrap();
//     PositiveNonzeroInteger::new(x)
//         .map_err(ParsePosNonzeroError::from_creation)
// }

// // Don't change anything below this line.

// #[derive(PartialEq, Debug)]
// struct PositiveNonzeroInteger(u64);

// #[derive(PartialEq, Debug)]
// enum CreationError {
//     Negative,
//     Zero,
// }

// impl PositiveNonzeroInteger {
//     fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
//         match value {
//             x if x < 0 => Err(CreationError::Negative),
//             x if x == 0 => Err(CreationError::Zero),
//             x => Ok(PositiveNonzeroInteger(x as u64))
//         }
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_parse_error() {
//         // We can't construct a ParseIntError, so we have to pattern match.
//         assert!(matches!(
//             parse_pos_nonzero("not a number"),
//             Err(ParsePosNonzeroError::ParseInt(_))
//         ));
//     }

//     #[test]
//     fn test_negative() {
//         assert_eq!(
//             parse_pos_nonzero("-555"),
//             Err(ParsePosNonzeroError::Creation(CreationError::Negative))
//         );
//     }

//     #[test]
//     fn test_zero() {
//         assert_eq!(
//             parse_pos_nonzero("0"),
//             Err(ParsePosNonzeroError::Creation(CreationError::Zero))
//         );
//     }

//     #[test]
//     fn test_positive() {
//         let x = PositiveNonzeroInteger::new(42);
//         assert!(x.is_ok());
//         assert_eq!(parse_pos_nonzero("42"), Ok(x.unwrap()));
//     }
// }

// I was not able to solve error_hadling6.rs, so I decided to revise error handling in the rust book

// Next: Matching on Different errors

// use std::fs::File;
// use std::io::ErrorKind;

// fn main(){
//     let greeting_file_open = File::open("hello.txt");
//     let greeting_file = match greeting_file_open {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Problem creating file {e:?}"),
//             },
//             _=>{ panic!("Problem opening the file: {error:?}");
//         }
//         },
//     };
// }

// next: Using the Unwrap and Expect:

// use std::fs::File;

// fn main(){
//     //let greeting_file = File::open("hello.txt").unwrap();
//     //Expect message has the same function as unwrap but also helps to convey the message
//     let greeting_file_open = File::open("hello.txt")
//         .expect("hello.txt should be included in this project"); 

// }


// Next: error Propagating.

// use std::fs::File;
// use std::io::{self, Read};

// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");

//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut username = String::new();

//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }


// errors6.rs

// Using catch-all error types like `Box<dyn error::Error>` isn't recommended
// for library code, where callers might want to make decisions based on the
// error content, instead of printing it out or propagating it further. Here,
// we define a custom error type to make it possible for callers to decide
// what to do next when our function returns an error.

// Make these tests pass! Execute `rustlings hint errors6` for hints :)

// I AM NOT DONE

// use std::num::ParseIntError;

// // This is a custom error type that we will be using in `parse_pos_nonzero()`.
// #[derive(PartialEq, Debug)]
// enum ParsePosNonzeroError {
//     Creation(CreationError),
//     ParseInt(ParseIntError)
// }

// impl From<ParseIntError> for ParsePosNonzeroError {
//     fn from(err: ParseIntError) -> Self {
//         ParsePosNonzeroError::ParseInt(err)
//     }
// }

// impl From<CreationError> for ParsePosNonzeroError {
//     fn from(err: CreationError) -> Self {
//         ParsePosNonzeroError::Creation(err)
//     }
// }

// fn parse_pos_nonzero(s: &str)
//     -> Result<PositiveNonzeroInteger, ParsePosNonzeroError>
// {
//     let x: i64 = s.parse()?;
//     let value = PositiveNonzeroInteger::new(x)?;
//     Ok(value)
   
//     // PositiveNonzeroInteger::new(x)
//     //     .map_err(ParsePosNonzeroError::from_creation)
// }

// // Don't change anything below this line.

// #[derive(PartialEq, Debug)]
// struct PositiveNonzeroInteger(u64);

// #[derive(PartialEq, Debug)]
// enum CreationError {
//     Negative,
//     Zero,
// }

// impl PositiveNonzeroInteger {
//     fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
//         match value {
//             x if x < 0 => Err(CreationError::Negative),
//             x if x == 0 => Err(CreationError::Zero),
//             x => Ok(PositiveNonzeroInteger(x as u64))
//         }
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_parse_error() {
//         // We can't construct a ParseIntError, so we have to pattern match.
//         assert!(matches!(
//             parse_pos_nonzero("not a number"),
//             Err(ParsePosNonzeroError::ParseInt(_))
//         ));
//     }

//     #[test]
//     fn test_negative() {
//         assert_eq!(
//             parse_pos_nonzero("-555"),
//             Err(ParsePosNonzeroError::Creation(CreationError::Negative))
//         );
//     }

//     #[test]
//     fn test_zero() {
//         assert_eq!(
//             parse_pos_nonzero("0"),
//             Err(ParsePosNonzeroError::Creation(CreationError::Zero))
//         );
//     }

//     #[test]
//     fn test_positive() {
//         let x = PositiveNonzeroInteger::new(42);
//         assert!(x.is_ok());
//         assert_eq!(parse_pos_nonzero("42"), Ok(x.unwrap()));
//     }
// } 


// Next: Modules.

// modules1.rs
// Make me compile! Execute `rustlings hint modules1` for hints :)

// I AM NOT DONE

// mod sausage_factory {
//     // Don't let anybody outside of this module see this!
//     fn get_secret_recipe() -> String {
//         String::from("Ginger")
//     }

//     pub fn make_sausage() {
//         get_secret_recipe();
//         println!("sausage!");
//     }
// }

// fn main() {
//     sausage_factory::make_sausage();
// }

// modules2.rs
// You can bring module paths into scopes and provide new names for them with the
// 'use' and 'as' keywords. Fix these 'use' statements to make the code compile.
// Make me compile! Execute `rustlings hint modules2` for hints :)

// I AM NOT DONE

// mod delicious_snacks {

//     // TODO: Fix these use statements
//     pub use self::fruits::PEAR as fruit;
//     pub use self::veggies::CUCUMBER as veggie;

//     mod fruits {
//         pub const PEAR: &'static str = "Pear";
//         pub const APPLE: &'static str = "Apple";
//     }

//     mod veggies {
//         pub const CUCUMBER: &'static str = "Cucumber";
//         pub const CARROT: &'static str = "Carrot";
//     }
// }

// fn main() {
//     println!(
//         "favorite snacks: {} and {}",
//         delicious_snacks::fruit,
//         delicious_snacks::veggie
//     );
// }


// modules3.rs
// You can use the 'use' keyword to bring module paths from modules from anywhere
// and especially from the Rust standard library into your scope.
// Bring SystemTime and UNIX_EPOCH
// from the std::time module. Bonus style points if you can do it with one line!
// Make me compile! Execute `rustlings hint modules3` for hints :)

// I AM NOT DONE

// TODO: Complete this use statement
// use std::time::SystemTime;
// use std::time::UNIX_EPOCH;

// fn main() {
//     match SystemTime::now().duration_since(UNIX_EPOCH) {
//         Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
//         Err(_) => panic!("SystemTime before UNIX EPOCH!"),
//     }
// }

// tests1.rs
// Tests are important to ensure that your code does what you think it should do.
// Tests can be run on this file with the following command:
// rustlings run tests1

// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests1` for hints :)

// I AM NOT DONE

// #[cfg(test)]
// mod tests {
//     #[test]
//         fn you_can_assert() {
//             assert!(3 > 4);
//     }
// }

// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests2` for hints :)

// I AM NOT DONE

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn you_can_assert_eq() {
//         assert_eq!(5,5);
//     }
// }


// tests3.rs
// This test isn't testing our function -- make it do that in such a way that
// the test passes. Then write a second test that tests whether we get the result
// we expect to get when we call `is_even(5)`.
// Execute `rustlings hint tests3` for hints :)

// I AM NOT DONE

// pub fn is_even(num: i32) -> bool {
//     num % 2 == 0
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn is_true_when_even() {
//         assert!(is_even(4));
//     }

//     #[test]
//     fn is_false_when_odd() {
//         assert!(!is_even(5));
//     }
// }


// traits1.rs
// Time to implement some traits!
//
// Your task is to implement the trait
// `AppendBar' for the type `String'.
//
// The trait AppendBar has only one function,
// which appends "Bar" to any object
// implementing this trait.

// I AM NOT DONE

// trait AppendBar {
//     fn append_bar(self) -> Self;
// }

// impl AppendBar for String {
//     fn append_bar(mut self) -> String {
//         self.push_str("Bar");
//         self
//     }
// }

// fn main() {
//     let s = String::from("Foo");
//     let s = s.append_bar();
//     println!("s: {}", s);
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn is_foo_bar() {
//         assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
//     }

//     #[test]
//     fn is_bar_bar() {
//         assert_eq!(
//             String::from("").append_bar().append_bar(),
//             String::from("BarBar")
//         );
//     }
// }


// traits2.rs
//
// Your task is to implement the trait
// `AppendBar' for a vector of strings.
//
// To implement this trait, consider for
// a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time,
// you can do this!

// I AM NOT DONE

// trait AppendBar {
//     fn append_bar(self) -> Self;
// }

// impl AppendBar for Vec<String>{
//     fn append_bar(mut self) -> Vec<String> {
//         for s in &mut self {
//             s.push_str("Bar");
//         }
//         self
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn is_vec_pop_eq_bar() {
//         let mut foo = vec![String::from("Foo")].append_bar();
//         assert_eq!(foo.pop().unwrap(), String::from("Bar"));
//         assert_eq!(foo.pop().unwrap(), String::from("Foo"));
//     }
// }

// I minted 10k tokens using solana cli and a local validator