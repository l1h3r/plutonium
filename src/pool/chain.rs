use nimiq_block::Block;
use nimiq_block::BlockHeader;
use nimiq_block::BlockInterlink;
use nimiq_block::Target;
use nimiq_hash::Blake2bHash;
use nimiq_hash::Hash;

#[derive(Debug)]
pub struct PoolChain {
  pub prev_block: Block,              // _poolPrevBlock
  pub bhash: Blake2bHash,             // _poolBodyHash
  pub ahash: Blake2bHash,             // _poolAccountsHash
  pub next_target: Target,            // _poolNextTarget
  pub next_interlink: BlockInterlink, // _poolNextInterlink
}

impl PoolChain {
  #[inline]
  pub fn next(&self, genesis: Blake2bHash, timestamp: u32) -> Block {
    self.block(genesis, timestamp)
  }

  #[inline]
  fn block(&self, genesis: Blake2bHash, timestamp: u32) -> Block {
    Block {
      body: None,
      interlink: self.next_interlink.clone(),
      header: self.header(genesis, timestamp),
    }
  }

  #[inline]
  fn header(&self, genesis: Blake2bHash, timestamp: u32) -> BlockHeader {
    BlockHeader {
      timestamp,
      nonce: 0,
      version: Block::VERSION,
      prev_hash: self.prev_block.header.hash(),
      interlink_hash: self.next_interlink.hash(genesis),
      body_hash: self.bhash.clone(),
      accounts_hash: self.ahash.clone(),
      n_bits: self.next_target.clone().into(),
      height: self.prev_block.header.height + 1,
    }
  }
}
