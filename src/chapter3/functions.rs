fn main(){
    println!("Hello Function World!");

    // first_function(5, "Arslan");
    let one_eight = second_function();
    println!("{}", one_eight)
}

                            //First simple function
// fn first_function(){
//     let x = "Functions";

//     println!("{}", x);
// }

                    // Parameters with a funciton;

// fn first_function(value: i64, person: &str) {
//     println!("The world is beautiful for number: {value} and {person} ");
// }


// New info: difference between statement and expression. Statement executes, return no value while 
//an Expression evaluate to a resultant value


                // Next will come a funciton that return a value

fn second_function() -> u32 {
    18
}
