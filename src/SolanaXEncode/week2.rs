// programmes do have single entry point in Solana

//for exmaple: 

pub fn process_instruction(
     _program_id: &Pubkey,
     _accounts: &[AccountInfo],
) {
   
}

//----------
// TO LEARN POST LESSON:
// Learn the nuacnes of serialisation and deserialisation
// Ideas from vitalik to build, just have a look

//-----------

//traits help to specify the behaviour

trait Details {
    fn get_owner(&self) -> &Pubkey;
    // fn get_admin(&self) --. etc
}

// implementations

imp Details for AccountA {
    gn get_owner(&self) -> &Pubkey {
        &self.owner 
    }
}


// Lesson 5 begins of real week 2: 

// within an account there wil be number of keys used: key, isSigner, isWritable, rent_epoch

// Owner versos holder

// holder- holds the private key of the account!!!
// It is holder who is allowed to transfer money
// owner can ammend the data of any account

// This is different from Ethereum.

// There are 2 data accounts: System owned account and PDA(Program Derived Accounts)

// Sealevel: PAraller processing of smart contracts

// Adds complexity
// each instruction tells the VM which account it wants to read and write in advance
// Ethereum is not here yet


// While you write programs, use modules. Multiple files.

// This is a best practice:
// lib.rs : Registering entrpoint.rs; instruction.rs(deserialisation of instruction)
// processor.rs (program logic), error.rs : Specific errors

// Rust - Lifetimes

//every single reference in Rust has a lifetime, whish is the
//scope for which that reference is valid.

//for exmaple: 

fn main(){
    let r;
    {
        let x = 5;
        r = &x;
    }

    println!("r: {}",r);
} // will  create an error oas x is out of scope after 


// ' is a lifetime annotation, for example &'i32; 


// when you are creating a project and limit the scope of the project to bare absolute minimum
// as people start usng your prokect it will change how you want to change and addnew functionality
//MVP


// Day 5 of the bootcamp: Trying to understand!!!
//-------
// Talk to joseph
// Talk to joseph about impermanent loss
// IL= change in the price of the token as out price changes
// Yield farming lending made availbale for the majority
// Flash Loans
// What is Composability:
// 




