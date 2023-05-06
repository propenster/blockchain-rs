#[macro_use]
extern crate serde_derive;
extern crate chrono;
mod blockchain;
mod shredder;
use blockchain::*;


use stopwatch::{Stopwatch};


fn main() {
    println!("Welcome to P2P Rust Blockchain experiment");

    println!("Running the Shredder function now...");

    let sw = Stopwatch::start_new();
    let process_file = shredder::shred().unwrap();
    println!("{:?}", process_file);
    println!("Processing 74GB CSVs took {}ms", sw.elapsed_ms()) //this took 2032ms

}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_genesis_block() {
        //create blockchain
        let p2p_bc: Vec<Block> = vec![Block::genesis()];
        assert_eq!(p2p_bc[0].block_number, 1);
        assert_eq!(
            p2p_bc[0].transaction_list[0].transaction_details,
            "This is dummy transaction as genesis block has no transactions"
        );
    }
    #[test]
    fn test_new_block() {
        let mut p2p_bc: Vec<Block> = vec![Block::genesis()];
        let new_txn = Transaction {
            transaction_id: String::from("1"),
            transaction_timestamp: 0,
            transaction_details: String::from("Testing a new transaction"),
        };
        let mut new_block = Block::new(vec![new_txn], &p2p_bc[p2p_bc.len() - 1]);
        Block::mine_new_block(&mut new_block, &PREFIX);
        p2p_bc.push(new_block);
        assert_eq!(p2p_bc.len(), 2);
        assert_eq!(
            p2p_bc[1].transaction_list[0].transaction_details,
            "Testing a new transaction"
        );
    }
}
