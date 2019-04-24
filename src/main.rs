extern crate time;
extern crate crypto;
extern crate hex;
extern crate json;

use self::crypto::digest::Digest;
use self::crypto::sha3::Sha3;
use ::hex::{FromHex, ToHex};

use ::time::*;
use std::rc::Rc;
use rustc_serialize::hex::FromHex;


struct Transaction {
    sender: String,
    recipient: String,
    amount: String,
}

struct Block {
    index: i8,
    timestamp: i64,
    transaction: Transaction,
    proof: i16,
    previous_hash: String,
}

struct Blockchain {
    current_transactions: Vec<Transaction>,
    chain: Vec<Block>,
    nodes: String,
}

impl Block {
    fn encode(&self) -> String {
        let mut str = String::new();
        str.push_str(&String::from(self.index));
        str.push_str(&String::from(self.timestamp));
        str.push_str(&self.transaction.sender);
        str.push_str(&self.transaction.recipient);
        str.push_str(&self.transaction.amount);
        str.push_str(&String::from(self.proof));
        str.push_str(&self.previous_hash);
        str
    }

    fn myhash(&self) -> String {
        // create a SHA3-256 object
        let mut hasher = Sha3::sha3_256();
        // write input message
        hasher.input_str(&self.encode());
        // read hash hexdigest
        hasher.result_str()
    }
}

impl Blockchain {
    fn new() -> Blockchain {
        Blockchain {
            current_transactions: Vec::new(),
            chain: Vec::new(),
            nodes: String::new(),
        }
    }

    fn init(&mut self) {
        self.new_block(100, Some(String::from("1")));
    }

    fn new_block(&mut self, proof: i16,
                 previous_hash: Option<String>) -> Block {
        self.current_transactions.clear();
        let new_block = Block {
            index: (self.chain.len() + 1) as i8,
            timestamp: timestamp(),
            transaction: self.transaction,
            proof,
            previous_hash: match previous_hash {
                None => {
                    myhash(self.chain.last())
                }
                Some(val) => val,
            },
        };
        self.chain.push(Rc::clone(&new_block));
        new_block
    }

    fn timestamp() -> i64 {
        let timespec = time::get_time();
        timespec.sec * 1000 + (timespec.nsec as f64 / 1000.0 / 1000.0) as i64
    }
}

fn main() {
    let mut blockchian = Blockchain::new();
    blockchian.init();
}