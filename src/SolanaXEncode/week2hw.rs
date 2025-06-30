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

fn main () {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    let mut my_iterable_fav_fruits = my_fav_fruits.iter();   // TODO: Step 1

    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));     // TODO: Step 2
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach"));     // TODO: Step 3
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));     // TODO: Step 4
}