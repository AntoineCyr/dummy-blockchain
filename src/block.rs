pub type Result<T> = std::result::Result<T,failure::Error>;
use serde::{Serialize,Deserialize};
use crypto::sha2::Sha256;
use crypto::digest::Digest;

#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct Block {
    transactions: Vec<Transaction>,
    prev_block_hash: String,
    hash: String,
    nonce: i32,
}

#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct Transaction {
    from: String,
    to: String,
    amount: i32,
}

impl Block {
    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    /// NewBlock creates and returns Block
    pub fn new_block(
        transactions: Vec<Transaction>,
        prev_block_hash: String,
    ) -> Block {
        let mut block = Block {
            transactions,
            prev_block_hash,
            hash: String::new(),
            nonce: 0,
        };
        let _ = block.find_hash();
        block
    }


    fn find_hash(&mut self) -> Result<()> {
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        self.hash = hasher.result_str();
        Ok(())
    }

    fn prepare_hash_data(&self) -> Result<Vec<u8>> {
        let content = (
            self.prev_block_hash.clone(),
            self.transactions.clone(),
            self.nonce,
        );
        let bytes = bincode::serialize(&content)?;
        Ok(bytes)
    }



}

impl Transaction {
    pub fn new(
        from:String,
        to:String,
        amount:i32
    ) -> Transaction {
        let transaction = Transaction {
            from,
            to,
            amount
        };
        transaction
    }

    pub fn get_to(&self) -> String {
        self.to.clone()
    }

    pub fn get_from(&self) -> String {
        self.from.clone()
    }

    pub fn get_amount(&self) -> i32 {
        self.amount.clone()
    }
}