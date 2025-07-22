use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub index: u32,
    pub timestamp: u64,
    pub event: String,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    /// Initialise une nouvelle blockchain avec un bloc Genesis
    pub fn new() -> Self {
        let mut blockchain = Blockchain { chain: vec![] };
        blockchain.add_block("Genesis Event".to_string(), "Welcome to Dream Chain".to_string());
        blockchain
    }

    /// Ajoute un bloc avec minage basique (nonce)
    pub fn add_block(&mut self, event: String, data: String) -> Block {
        let index = self.chain.len() as u32;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let previous_hash = self.chain
            .last()
            .map_or_else(|| "0".to_string(), |b| b.hash.clone());

        let mut nonce = 0;
        let mut hash = Self::calculate_hash(index, timestamp, &event, &data, &previous_hash, nonce);

        // Minage: recherche d'un hash commençant par "0000"
        while &hash[..4] != "0000" {
            nonce += 1;
            hash = Self::calculate_hash(index, timestamp, &event, &data, &previous_hash, nonce);
        }

        let block = Block {
            index,
            timestamp,
            event,
            data,
            previous_hash,
            hash,
            nonce,
        };

        self.chain.push(block.clone());
        block
    }

    /// Retourne une référence à la chaîne
    pub fn get_chain(&self) -> &Vec<Block> {
        &self.chain
    }

    /// Calcule le hash d'un bloc
    fn calculate_hash(
        index: u32,
        timestamp: u64,
        event: &str,
        data: &str,
        previous_hash: &str,
        nonce: u64,
    ) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{index}{timestamp}{event}{data}{previous_hash}{nonce}"));
        format!("{:x}", hasher.finalize())
    }
}
