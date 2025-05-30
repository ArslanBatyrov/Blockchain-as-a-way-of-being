use crate::block::Block;

fn main(){
    let difficulty = 0x000fffffffffffffffffffffffffffff;
    let mut genesis_block = Block::new(
        0, 
        now(),  
        vec![0;32],
        vec![Transaction {
            inputs:  vec![],
            otputs: vec![
                transaction::Output{
                    to_addr:"Arslan".to_owned(),
                    value: 50,
                },
                transaction::Output{
                    to_addr:"Joseph".to_owned(),
                    value: 7,
                },
            ],

        },], difficulty);
}