use hex::encode;
use sha2::{Sha256, Digest};
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct MerkleNode {
    pub hash: Vec<u8>,
    pub left: Option<Box<MerkleNode>>,
    pub right: Option<Box<MerkleNode>>
}

pub struct MerkleProof {
    pub node: Box<MerkleNode>,
    pub direction: bool
}

#[derive(Debug)]
pub struct MerkleTree {
    pub root: Option<Box<MerkleNode>>
}

// Move to other place
fn create_hash(data: impl AsRef<[u8]>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);

    hasher.finalize().to_vec()
}

impl MerkleNode {
    pub fn new(
        left: Option<Box<MerkleNode>>,
        right: Option<Box<MerkleNode>>,
        hash: Vec<u8>,
    ) -> Self {
        let mut node = MerkleNode {
            hash,
            left: None,
            right: None,
        };

        if let (Some(left), Some(right)) = (&left, &right) {
            let mut hasher = Sha256::new();

            hasher.update(&left.hash);
            hasher.update(&right.hash);

            node.hash = hasher.finalize().to_vec();
            // node.hash = [left.hash.clone(), right.hash.clone()].concat();
        }

        node.left = left;
        node.right = right;

        node
    }
}

impl MerkleProof {
    pub fn new(node: MerkleNode, direction: bool) -> MerkleProof {
        MerkleProof {
            node: Box::new(node),
            direction,
        }
    }
}

impl MerkleTree {
    pub fn new(data: Vec<Vec<u8>>) -> MerkleTree {
        let mut hashes = Vec::new();

        for d in data {
            let mut hasher = Sha256::new();
            hasher.update(d.to_vec());
            hashes.push(hasher.finalize().to_vec());
            // hashes.push(d);
        }

        if hashes.len() % 2 != 0 {
            println!("Odd number of hashes");
            hashes.push(hashes.last().unwrap().clone());
        }

        // if data.len() % 2 != 0 {
        //     println!("Odd number of hashes");
        //     data.insert(0, data.last().unwrap().clone());
        // }

        let mut queue: VecDeque<MerkleNode> = VecDeque::new();

        queue.extend(hashes.into_iter().map(|hash| MerkleNode::new(None, None, hash)));
        // queue.extend(data.into_iter().map(|hash| MerkleNode::new(None, None, hash)));

        while queue.len() > 1 {
            let left = queue.pop_front().unwrap();
            let right = queue.pop_front().unwrap();

            let mut hasher = Sha256::new();
            hasher.update(&left.hash);
            hasher.update(&right.hash);

            let hash = hasher.finalize().to_vec();
            // let hash = [left.hash.clone(), right.hash.clone()].concat();

            let node = MerkleNode::new(Some(Box::new(left)), Some(Box::new(right)), hash);

            queue.push_back(node);
        }

        let root = queue.pop_front().unwrap();

        MerkleTree {
            root: Some(Box::new(root)),
        }
    }

    pub fn get_proof(&self, node_hash: &Vec<u8>) -> Vec<MerkleProof> {
        let mut proof: Vec<MerkleProof> = Vec::new();
        self._get_proof(&self.root, node_hash, &mut proof);
        proof
    }

    fn _get_proof(
        &self,
        node: &Option<Box<MerkleNode>>,
        node_hash: &Vec<u8>,
        proof: & mut Vec<MerkleProof>
    ) -> bool {
        match node {
            Some(node) => {
                println!();
                println!("===== Comparison =====");
                println!();
                println!("Hash target {:?}", hex::encode(node_hash));
                println!("Hash comparison {:?}", hex::encode(node.hash.clone()));
                // println!("Hash target {:?}", std::string::String::from_utf8(node_hash.to_vec()).unwrap());
                // println!("Hash comparison {:?}", std::string::String::from_utf8(node.hash.clone()).unwrap());
                println!();

                if node.hash == *node_hash {
                    return true;
                }
                
                if self._get_proof(&node.left, node_hash, proof) {
                    if let Some(right) = &node.right {
                        proof.push(MerkleProof::new(*right.clone(), true));
                    }
                    return true;
                }

                if self._get_proof(&node.right, node_hash, proof) {
                    if let Some(left) = &node.left {
                        proof.push(MerkleProof::new(*left.clone(), false));
                    }
                    return true;
                }

                false
            },
            None => false,
        }
    }

    pub fn validate_proof(&self, proof: Vec<MerkleProof>, hash_to_prove: Vec<u8>) -> bool {
        println!("");
        println!("===== List of Proof =====");
        println!("");

        for p in proof.iter() {
            println!("Node: {:?}", hex::encode(&p.node.hash));
            // println!("Node: {:?}", std::string::String::from_utf8(p.node.hash.to_vec()).unwrap());
            println!("Direction: {:?}", if p.direction { "Right" } else { "Left" });
        }

        println!("");
        println!("===== Proof =====");
        println!("");

        let mut hash = hash_to_prove;

        for p in proof.iter() {
            if p.direction {
                // println!("Merging hash {:?} with {:?}", std::string::String::from_utf8(hash.clone()).unwrap(), std::string::String::from_utf8(p.node.hash.to_vec()).unwrap());
                println!("Merging hash {:?} with {:?}", encode(hash.clone()), encode(p.node.hash.to_vec()));
                // hash = [hash, p.node.hash.to_vec()].concat();
                hash = create_hash(&[hash, p.node.hash.to_vec()].concat());
            } else {
                // println!("Merging hash {:?} with {:?}", std::string::String::from_utf8(p.node.hash.to_vec()).unwrap(), std::string::String::from_utf8(hash.clone()).unwrap());
                println!("Merging hash {:?} with {:?}", encode(p.node.hash.to_vec()), encode(hash.clone()));
                // hash = [p.node.hash.to_vec(), hash].concat();
                hash = create_hash(&[p.node.hash.to_vec(), hash].concat());
            }
        }

        let root_hash = self.root.as_ref().unwrap().hash.to_vec();

        println!("Expected hash: {:?}", root_hash);
        // println!("Expected hash: {:?}", std::string::String::from_utf8(root_hash.clone()).unwrap());
        println!("Expected hash: {:?}", hex::encode(&root_hash));
        println!("");
        println!("Calculated hash: {:?}", hash);
        // println!("Calculated hash: {:?}", std::string::String::from_utf8(hash.clone()).unwrap());
        println!("Calculated hash: {:?}", hex::encode(&hash));

        root_hash == hash
    }

    pub fn print(&self) {
        // println!("Root => {:?}", std::string::String::from_utf8(self.root.as_ref().unwrap().hash.clone()).unwrap());
        println!("Root => {:?}", hex::encode(&self.root.as_ref().unwrap().hash));
        println!("");
        self._print(&self.root, 0);
    }

    fn _print(&self, node: &Option<Box<MerkleNode>>, space: usize) {
        if let Some(node) = node {
            self._print(&node.right, space + 4);
            // println!("{:space$}{:?}", "", std::string::String::from_utf8(node.hash.clone()).unwrap(), space = space);
            println!("{:space$}{:?}", "", hex::encode(&node.hash), space = space);
            self._print(&node.left, space + 4);
        }
    }
}

