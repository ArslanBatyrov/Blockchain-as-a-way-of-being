// Next: Lesson 9. I start my week 3 revision

// I ahve realised that SolanaXEncode explanation of anchor is really bad
// So I started watching the Solana Bootcamp 11hours video to get deeper understanding of the Anchor
// As I was watching the video, I realised that I need to dive deeper into lifetimes


// Next: Lifetimes

// fn main(){
//     let r;

//     let x =5;
//     r = &x;
//     println!("r: {}", r);
//     // This won't cause na error as lifetimes are valid.

    
    
// }

// fn main(){
//     let string1 = String::from("Hello");
//     let string2 = String::from("Worlddd");

//     let result = longest(string1.as_str(), string2.as_str());
//     println!("The longest string is: {}", result);
// }

// fn longest(x: str, y: str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }  // to solve this error it requires us to use a gen. lifetime annotation

fn main(){
    let string1 = String::from("Hello");
    let string2 = String::from("Worlddd");

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is: {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
} // generic lifetimes to dot change the real lifetimes, they only specify that there is a relationship
// between lifetimes. In this case with a, they specify that the lifetime of the return value
// is the smallest value between the two inputs : x and y.