use base64::decode;
use beserial::Deserialize;
use nimiq_block::Block;
use nimiq_block::BlockInterlink;
use nimiq_block::Target;
use nimiq_blockchain::Blockchain;
use nimiq_hash::Blake2bHash;
use nimiq_hash::Hash;
use nimiq_network::Network;
use nimiq_network_primitives::networks::get_network_info;
use nimiq_network_primitives::networks::NetworkInfo;
use nimiq_primitives::networks::NetworkId;
use std::sync::Arc;

use crate::error::Error;
use crate::miner::MINER;
use crate::pool::PoolChain;
use crate::pool::PoolClient;
use crate::pool::PoolConfig;
use crate::pool::PoolMessage;

type ArcChain = Arc<Blockchain<'static>>;
type ArcNetwork = Arc<Network>;

const TAG: &'static str = "PoolMiner";

pub struct PoolMiner {
  blockchain: ArcChain,
  network: ArcNetwork,
  config: PoolConfig,
  enabled: bool, // _miningEnabled
  connected: bool,
  hashrate: u32, // _hashrate
  pool: Option<PoolChain>,
}

impl std::fmt::Debug for PoolMiner {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    f.debug_struct("PoolMiner")
      .field("config", &self.config)
      .field("enabled", &self.enabled)
      .field("connected", &self.connected)
      .field("hashrate", &self.hashrate)
      .finish()
  }
}

impl PoolMiner {
  pub fn new(blockchain: ArcChain, network: ArcNetwork, config: PoolConfig) -> Self {
    let _ = &*MINER;

    Self {
      blockchain,
      network,
      config,
      enabled: false,
      connected: false,
      hashrate: 0,
      pool: None,
    }
  }

  #[inline]
  pub fn client(&self) -> PoolClient {
    PoolClient::new(&self.config).unwrap()
  }

  pub fn connect(&mut self) {
    self.connected = true;
  }

  pub fn disconnect(&mut self) {
    self.connected = false;
  }

  pub fn start(&mut self) {
    if self.enabled {
      println!("[{}] Already Enabled", TAG);
      return;
    }

    self.start_mining();
  }

  pub fn process(&mut self, message: PoolMessage) -> Result<(), Error> {
    let mut stop: bool = false;

    match message {
      PoolMessage::NewBlock {
        body,
        accounts,
        previous,
      } => {
        let previous: Block = Block::deserialize_from_vec(&mut decode(&previous)?)?;
        let previous_hash: Blake2bHash = previous.header.hash();
        let head_hash: Blake2bHash = self.blockchain.head_hash();
        let head_block: &Block = &*self.blockchain.head();

        println!("[{}] New base block from pool server, on top of {}", TAG, previous_hash);
        println!("[{}] - previous = {}", TAG, previous_hash);
        println!("[{}] - head     = {}", TAG, head_hash);

        let mut next: Option<Target> = None;

        if head_hash == previous_hash {
          // We are on the same head, that's great.
          next = Some(self.blockchain.get_next_target(None));
        } else if head_hash == previous.header.prev_hash {
          // We don't know the new block yet, make sure it's kinda valid.
          if previous.is_immediate_successor_of(head_block) {
            next = Some(self.blockchain.get_next_target(Some(&head_hash)));
          // this._poolNextTarget = await blockchain.getNextTarget(blockchain.head, previous);
          } else {
            println!(
              "[{}] {} (from pool) is not an immediate successor of {}, but is announced as such.",
              TAG, previous_hash, head_hash,
            );

            stop = true;
          }
        } else if head_block.header.prev_hash == previous_hash {
          // Pool does not know the new block yet, waiting for it.
          stop = true;
        } else {
          let previous2: Option<Block> = self.blockchain.get_block(&previous.header.prev_hash, false, false);

          if previous2.is_some() && self.blockchain.height() == previous.header.height {
            let previous2: Block = previous2.unwrap();
            let previous2_hash: Blake2bHash = previous2.header.hash();

            // Pool is on a different fork of length 1 and we want to please our pool
            if !previous.is_immediate_successor_of(&previous2) {
              println!(
                "[{}] {} (from pool) is not an immediate successor of {}, but is announced as such.",
                TAG, previous_hash, previous2_hash,
              );

              stop = true;
            }
          } else if self
            .blockchain
            .get_block(&previous.header.prev_hash, true, false)
            .is_some()
          {
            // Pool mines a fork
            println!(
              "[{}] {} (from pool) is a known fork, we don't mine on forks.",
              TAG, previous_hash,
            );

            stop = true;
          } else {
            println!(
              "[{}] {} (from pool) is unknown and not a successor of the head",
              TAG, previous_hash,
            );

            stop = true;
          }
        }

        if let Some(next) = next {
          let next_interlink: BlockInterlink = previous.get_next_interlink(&next);

          self.pool = Some(PoolChain {
            next_interlink,
            next_target: next,
            prev_block: previous,
            ahash: Blake2bHash::deserialize_from_vec(&mut decode(&accounts)?)?,
            bhash: Blake2bHash::deserialize_from_vec(&mut decode(&body)?)?,
          });
        }
      }
      PoolMessage::Registered => {
        println!("[+] Pool Registration Complete");
      }
      PoolMessage::Settings {
        address,
        extra,
        target,
        nonce,
        ..
      } => {
        println!("[+] Pool Settings");
        println!("[+] - address = {}", address);
        println!("[+] - extra   = {}", extra);
        println!("[+] - target  = {}", target);
        println!("[+] - nonce   = {}", nonce);

        MINER.write().unwrap().scompact(target);
      }
      PoolMessage::Balance {
        balance,
        confirmed,
        payout,
      } => {
        println!("[+] Pool Balance");
        println!("[+] - current   = {}", balance);
        println!("[+] - confirmed = {}", confirmed);
        println!("[+] - payout    = {}", payout);
      }
      PoolMessage::Error { reason } => {
        eprintln!("[x] Pool Error: {}", reason);
      }
      message => {
        println!("[{}] Unknown Pool Message: {:#?}", TAG, message);
      }
    }

    if stop {
      self.stop_mining();
    } else {
      self.start_mining();
    }

    Ok(())
  }

  fn start_mining(&mut self) {
    if self.pool.is_none() {
      return;
    }

    let pool: &PoolChain = self.pool.as_ref().unwrap();

    self.enabled = true;

    let info: &NetworkInfo = get_network_info(NetworkId::Main).unwrap();
    let hash: Blake2bHash = info.genesis_block.header.hash();

    let now: u64 = self.network.network_time.now();
    let now: u32 = (now / 1000) as u32;
    let time: u32 = now.max(self.blockchain.head().header.timestamp + 1);
    let time: u32 = time.max(pool.prev_block.header.timestamp + 1);

    // Construct next block.
    let block = pool.next(hash, time);

    println!(
      "[{}] Starting work on block #{} ({} H/s)",
      TAG, block.header.height, self.hashrate,
    );

    MINER
      .write()
      .unwrap()
      .mine(block, Arc::clone(&self.blockchain))
      .unwrap();
  }

  fn stop_mining(&mut self) {
    self.enabled = false;
    // TODO: So much...
  }
}
