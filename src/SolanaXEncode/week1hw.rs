//Here I will be revising the enetire week 1

// tuples Revision:
// fn main (){
//     let pair: (i32, &str) = (100, "Rust");
//     let (num, text) = pair;
//     println!("Number: {}", pair.0);
//     println!("Text: {}", pair.1);
// }



// Next: Structs Revision

struct Man {
    height: u64,
    name: String,
    age: u64,
}


impl Man {
    fn birthday(&mut self){
      self.age += 1;
      println!("Happy {}th birthday, {}!", self.age, self.name);  
    }
}

fn main() {
    let mut arslan = Man { name: "Arslan".into(), age: 20, height: 188,};
    arslan.birthday();
}