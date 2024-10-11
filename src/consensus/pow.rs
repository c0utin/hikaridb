use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};
use sha256::digest;
use std::fmt::Write;

#[derive(Debug)]
pub struct Block<'a> {
    index: i32,
    timestamp: u64,
    prev_hash: &'a str,
    data: &'a str,
    nonce: i32,
    difficulty: i32,
    hash: String,
}

impl<'a> fmt::Display for Block<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Block {{ index: {}, timestamp: {}, prev_hash: {}, data: {}, nonce: {}, difficulty: {}, hash: {} }}",
               self.index, self.timestamp, self.prev_hash, self.data, self.nonce, self.difficulty, self.hash)
    }
}

impl<'a> Block<'a> {
    pub fn new(
        index: i32,
        timestamp: u64,
        prev_hash: &'a str,
        data: &'a str,
        nonce: i32,
        difficulty: i32,
    ) -> Self {
        let mut block = Block {
            index,
            timestamp,
            prev_hash,
            data,
            nonce,
            difficulty,
            hash: String::new(),
        };

        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let mut record = String::new();
        write!(
            &mut record,
            "{}{}{}{}{}",
            self.index, self.timestamp, self.prev_hash, self.data, self.nonce
        )
        .unwrap();

        let hasher = digest(record);
        hasher
    }
}

pub fn generate_block_with_pow<'a>(
    prev_block: &'a Block,
    data: &'a str,
    difficulty: i32,
) -> Block<'a> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut nonce = 0;

    let mut new_block = Block::new(
        prev_block.index + 1,
        timestamp,
        &prev_block.hash,
        data,
        nonce,
        difficulty,
    );

    while !new_block.hash.starts_with(&"0".repeat(difficulty as usize)) {
        nonce += 1;
        new_block.nonce = nonce;
        new_block.hash = new_block.calculate_hash();
        println!("Nonce: {}, Hash: {}", nonce, new_block.hash);
    }

    new_block
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    // Helper function to create a dummy previous block
    fn create_genesis_block<'a>() -> Block<'a> {
        Block::new(
            0,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            "0",
            "Genesis Block",
            0,
            2, // difficulty of 2 (example)
        )
    }

    #[test]
    fn test_block_creation() {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let block = Block::new(1, timestamp, "0", "Test Data", 0, 2);

        assert_eq!(block.index, 1);
        assert_eq!(block.timestamp, timestamp);
        assert_eq!(block.prev_hash, "0");
        assert_eq!(block.data, "Test Data");
        assert_eq!(block.nonce, 0);
        assert_eq!(block.difficulty, 2);
        assert_eq!(block.hash, block.calculate_hash());
    }

    #[test]
    fn test_calculate_hash() {
        let block = Block::new(1, 1234567890, "0", "Test Data", 0, 2);
        let expected_hash = block.calculate_hash();
        
        // Hash must be valid and not empty
        assert_eq!(block.hash, expected_hash);
        assert!(!block.hash.is_empty());
    }

    #[test]
    fn test_proof_of_work() {
        let genesis_block = create_genesis_block();
        let difficulty = 2;
        let new_block = generate_block_with_pow(&genesis_block, "New Block Data", difficulty);

        // Hash must start with the correct number of leading zeros based on difficulty
        assert!(new_block.hash.starts_with(&"0".repeat(difficulty as usize)));

        // Ensure the block's data matches expectations
        assert_eq!(new_block.index, genesis_block.index + 1);
        assert_eq!(new_block.prev_hash, genesis_block.hash);
        assert_eq!(new_block.data, "New Block Data");
    }
}

