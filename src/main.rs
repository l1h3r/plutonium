use nimiq_blockchain::Blockchain;
use nimiq_blockchain::BlockchainEvent;
use nimiq_consensus::consensus::Consensus;
use nimiq_consensus::consensus::ConsensusEvent;
use nimiq_database::lmdb::open::Flags;
use nimiq_database::lmdb::LmdbEnvironment;
use nimiq_database::Environment;
use nimiq_keys::Address;
use nimiq_keys::KeyPair;
use nimiq_lib::client::Client;
use nimiq_lib::client::ClientBuilder;
use nimiq_lib::client::ClientInitializeFuture;
use nimiq_network::network_config::Seed;
use nimiq_network::Network;
use nimiq_network_primitives::address::PeerAddress;
use nimiq_network_primitives::address::PeerUri;
use nimiq_network_primitives::networks::get_network_info;
use nimiq_network_primitives::networks::NetworkInfo;
use nimiq_network_primitives::protocol::Protocol;
use nimiq_primitives::networks::NetworkId;

use futures::Future;
use futures::IntoFuture;
use futures::Stream;
use std::str::FromStr;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::RwLock;
use tokio::run;

use plutonium::error::Error;
use plutonium::pool::PoolClient;
use plutonium::pool::PoolConfig;
use plutonium::pool::PoolMiner;

const MINER_VERSION: &'static str = concat!("GPU Miner ", env!("CARGO_PKG_VERSION"));

const TAG: &'static str = "SushiPoolMiner";

static ESTABLISHED: AtomicBool = AtomicBool::new(false);

fn main() -> Result<(), Error> {
  let config: PoolConfig = PoolConfig::default();

  // TODO: Handle error
  let env: Environment = LmdbEnvironment::new(config.db_path, config.db_size, config.db_max, Flags::empty()).unwrap();

  let env: &'static Environment = Box::leak(Box::from(env));
  let mut builder: ClientBuilder = ClientBuilder::new(Protocol::Ws, env); // TODO: Protocol::Dumb?
  let info: &NetworkInfo = get_network_info(NetworkId::Main).unwrap();

  let seeds: Vec<Seed> = info
    .seed_peers
    .iter()
    .map(|addr| addr.to_seed_string().unwrap())
    .map(|uri| PeerUri::from_str(&uri).unwrap())
    .map(Seed::Peer)
    .collect();

  builder.with_network_id(NetworkId::Main);
  builder.with_hostname(config.host);
  builder.with_port(config.port);
  builder.with_seeds(seeds);

  let client: ClientInitializeFuture = builder.build_client()?;
  let consensus: Arc<Consensus> = client.consensus();

  let blockchain: Arc<Blockchain<'static>> = Arc::clone(&consensus.blockchain);
  let network: Arc<Network> = Arc::clone(&consensus.network);

  // const deviceId = Nimiq.BasePoolMiner.generateDeviceId(networkConfig);
  // TODO: Handle error
  let address: Address = Address::from_user_friendly_address(config.address).unwrap();
  let peer: PeerAddress = network.network_config.peer_address();
  let pair: &KeyPair = network.network_config.key_pair();

  println!("[{}] Sushipool {} starting", TAG, MINER_VERSION);
  println!("[{}] - pool server  = {}:{}", TAG, config.host, config.port);
  println!("[{}] - peer address = {}", TAG, peer);
  println!("[{}] - peer pub key = {}", TAG, pair.public.to_hex());
  println!("[{}] - device       = {}", TAG, config.name);
  println!("[{}] - address      = {}", TAG, address.to_user_friendly_address());

  let host = config.host;

  let miner: PoolMiner = PoolMiner::new(Arc::clone(&blockchain), Arc::clone(&network), config);

  let miner: Arc<RwLock<PoolMiner>> = Arc::new(RwLock::new(miner));

  // $.miner = new NanoPoolMiner($.blockchain, $.network.time, address, deviceId, deviceData, config.devices, config.memory);

  // $.miner.on('share', (block, blockValid) => {
  //   Log.i(TAG, `Found share. Nonce: ${block.header.nonce}`);
  // });

  // $.miner.on('hashrates-changed', hashrates => {
  //   const totalHashRate = hashrates.reduce((a, b) => a + b);
  //   const gpuInfo = $.miner.gpuInfo;
  //   Log.i(TAG, `Hashrate: ${humanHashrate(totalHashRate)} | ${hashrates.map((hr, idx) => `GPU${gpuInfo[idx].idx}: ${humanHashrate(hr)}`).join(' | ')}`);
  // });

  //
  // Consensus Events
  //

  {
    let miner: Arc<RwLock<PoolMiner>> = Arc::clone(&miner);
    let consensus: Arc<Consensus> = Arc::clone(&consensus);
    let network: Arc<Network> = Arc::clone(&network);

    consensus.notifier.write().register(move |event: &ConsensusEvent| {
      println!("[{}] Consensus Event: {:?}", TAG, event);

      match event {
        ConsensusEvent::Established => {
          println!("[{}] Connecting to {}", TAG, host);
          println!("[{}] Peers = {}", TAG, network.peer_count());

          ESTABLISHED.store(true, Ordering::SeqCst);

          miner.write().unwrap().connect();
        }
        ConsensusEvent::Lost => {
          println!("[{}] Lost connection to {}", TAG, host);
          println!("[{}] Peers = {}", TAG, network.peer_count());

          ESTABLISHED.store(false, Ordering::SeqCst);

          miner.write().unwrap().disconnect();
        }
        ConsensusEvent::Syncing => {
          println!("[{}] Consensus Syncing...", TAG);
          println!("[{}] Peers = {}", TAG, network.peer_count());
        }
        ConsensusEvent::Waiting => {
          println!("[{}] Consensus Waiting...", TAG);
          println!("[{}] Peers = {}", TAG, network.peer_count());
        }
        ConsensusEvent::SyncFailed => {
          println!("[{}] Consensus Sync Failed...", TAG);
          println!("[{}] Peers = {}", TAG, network.peer_count());
        }
      }
    });
  }

  //
  // Blockchain Events
  //

  {
    let chain: Arc<Blockchain<'static>> = Arc::clone(&blockchain);

    blockchain.notifier.write().register(move |event: &BlockchainEvent| {
      let log: bool = chain.height() % 100 == 0 || ESTABLISHED.load(Ordering::Acquire);

      match event {
        BlockchainEvent::Extended(_hash) => {
          if log {
            println!("[{}] Blockchain Extended = {}", TAG, chain.height());
            println!("[{}] Latest Block = https://nimiq.watch/#{}", TAG, chain.head_hash());
            println!("[{}] Peers = {}", TAG, network.peer_count());
          }
        }
        BlockchainEvent::Rebranched(_reverted, _adopted) => {
          if log {
            println!("[{}] Blockchain Rebranched = {}", TAG, chain.height());
            println!("[{}] Latest Block = https://nimiq.watch/#{}", TAG, chain.head_hash());
            println!("[{}] Peers = {}", TAG, network.peer_count());
          }
        }
      }
    });
  }

  //
  // Network Events
  //

  // TODO: FIXME. When we listen for these events, we disable consensus sync...
  // not sure if there is a better way to handle this or just Nimiq core-rs bug
  // {
  //   network.notifier.write().register(move |event: NetworkEvent| {
  //     match event {
  //       NetworkEvent::PeerJoined(peer) => {
  //         println!("[{}] Connected to {}", TAG, peer);
  //       }
  //       NetworkEvent::PeerLeft(peer) => {
  //         println!("[{}] Disconnected from {}", TAG, peer);
  //       }
  //       NetworkEvent::PeersChanged => {
  //         println!("[{}] Peers Changed", TAG);
  //       }
  //     }
  //   });
  // }

  println!("[{}] Connecting to Nimiq network", TAG);

  //
  // Connect
  //

  {
    let miner: Arc<RwLock<PoolMiner>> = Arc::clone(&miner);
    let pclient: PoolClient = miner.write().unwrap().client();

    let stream = pclient
      .for_each(move |message| miner.write().unwrap().process(message))
      .map_err(|error| println!("Error: {:?}", error))
      .into_future();

    let future = client
      .and_then(|client| client.connect()) // Run Nimiq client
      .map(|_| println!("[+] Client finished")) // Map Result to None
      .map_err(|error| println!("[x] Client failed: {}", error))
      .and_then(move |_| stream)
      .map(|_| println!("Other futures finished"));

    run(future);
  }

  Ok(())
}
