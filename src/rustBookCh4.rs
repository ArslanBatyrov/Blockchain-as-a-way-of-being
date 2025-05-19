// Here I will be covering ownership
// One of the most unique features.



       {                    // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
               }           // this scope is now over, and s is no longer valid



// this was an example of the string that can be mutated, covering memory allocation

 {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }                                  // this scope is now over, and s is no
                                       // longer valid