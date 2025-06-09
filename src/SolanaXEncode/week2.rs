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





