mod models;

use models::blockchain::Blockchain;

fn main() {
    let difficulty = 1;
    let mut blockchain = Blockchain::new(difficulty);

    // Add some example blocks to the blockchain
    blockchain.add_block("Block 1 data".to_string());
    blockchain.add_block("Block 2 data".to_string());

    // Print the blockchain
    println!("{:?}", blockchain);
}
