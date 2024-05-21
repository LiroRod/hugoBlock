mod models;

use models::blockchain::Blockchain;
use models::merkle::MerkleTree;
use sha2::{Sha256, Digest};

//temp
fn create_hash(data: impl AsRef<[u8]>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);

    hasher.finalize().to_vec()
}

fn main() {
    let difficulty = 1;
    let mut blockchain = Blockchain::new(difficulty);

    println!("");

    // Add some example blocks to the blockchain
    blockchain.add_block("Block 1 data".to_string());
    blockchain.add_block("Block 2 data".to_string());

    // Print the blockchain
    println!("");
    println!("===== Blockchain =====");
    println!("");
    println!("{:?}", blockchain);
    println!("");

    let data_to_validate = "hello".as_bytes().to_vec();
    
    // Add some example hashes to the merkle tree
    let data = vec![
        data_to_validate.clone(),
        "world".as_bytes().to_vec(),
        "blockchain".as_bytes().to_vec(),
        "merkle".as_bytes().to_vec(),
        "another".as_bytes().to_vec(),
        "word".as_bytes().to_vec(),
    ];

    // for i in 0..1024 {
    //     data.push(format!("random-word-{}", i).as_bytes().to_vec());
    // }
    
    println!("");
    println!("===== Merkle Tree =====");
    println!("");

    // Create a new merkle tree
    let merkle_tree = MerkleTree::new(data);

    // Print the merkle tree
    println!("");
    merkle_tree.print();
    println!("");

    // Prove that the merkle tree is working

    println!("");
    println!("===== Retrieving Proof =====");
    println!("");
    
    let proof = merkle_tree.get_proof(&create_hash(data_to_validate.clone()));
    // let proof = merkle_tree.get_proof(&"hello".as_bytes().to_vec());

    let is_valid = merkle_tree.validate_proof(proof, create_hash(data_to_validate));

    println!("");

    println!("Valid: {:?}", is_valid);
}

