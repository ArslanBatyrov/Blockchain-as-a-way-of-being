                // exploring if expressions in Rust

fn main_if() {
    let number = 12;

//interesting thing to note for if expressions is that the condition must be a bool
    if number < 5 {
       println!("number is less than 5");
    } else if number < 10{
        println!("number is less than 10");
    } else{
         println!("number is greater than 10");
    }
}


            // exploring loops in Rust


            //This loop will be printing infinitely


// fn main_loops() {
//     let mut counter = 0;

//     let result = 'my_first_loop_label: loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {result}");
// }


                //Condiitonal loops

// fn main(){
//     let mut number = 5;

//     while number != 0 {
//         println!("{number}");

//         number -= 1;
//     }

//     println!("LIFTOFFFFFFFF!")
// }


            // looping though Arrays here
// fn main(){
//     let a = [10,50,100,200];
    
//     let mut index = 0;

//     while index < 5 {
//         println!("Your value is: {}", a[index]);

//         index +=1;
//     }
// }

            // looping though Array: Way 2


fn main(){
    let a = [10,50,100,200];

    for element in a {
        println!("the value is: {element}");
    }
}


// Ch3 is officially completeD!s 