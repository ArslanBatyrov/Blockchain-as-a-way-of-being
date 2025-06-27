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

fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}

fn sale_price(price: i32) -> i32 {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}  //after the arrow -> you must specify the type you want to return