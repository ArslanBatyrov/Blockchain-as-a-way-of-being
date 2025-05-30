use std::fmt::{self,Debug, Formatter};

pub type Hash = [u8; u32];

pub struct Block<T> {
    pub index: u32, 
    pub timestamp: u128, 
    pub hash: Hash, 
    pub prev_block_hash: Hash,
    pub nonce: u64,
    pub transactions: Vec<T>,
    pub difficulty: u128,
}

impl<T> Block<T> {
    pub fn new (
        index: u32,
        timestamp: u128,
        prev_block_hash: Hash,
        transactions: Vec<T>,
        difficulty:  u128) -> Self{
            Block{  
                index,
                timestamp,
                hash: [0u8; 32],
                prev_block_hash,
                nonce: 0,
                transactions,
                difficulty,
                }
        }
}

impl Debug for Block<T> {
    fn fmt(&self, f:&mut Formatter) -> fmt::Result{
        write!(f, "Block[{}]:{},at:{},with:{},nonce:{}",
        &self.index,
        &hex::encode(&self.hash),
        &self.timestamp,
        &self.transactions.len(),
        &self.nonce,
    )
    }
}