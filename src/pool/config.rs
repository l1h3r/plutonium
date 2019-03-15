use url::Url;

const DESIRED_SPS: u32 = 5;

// Nimiq Acct: NQ33 G0T9 D63A TMN2 S18B 960Q S3Q3 TMYD KQJE
// Nimiq Pin: 243915

#[derive(Debug)]
pub struct PoolConfig {
  /// Wallet address
  pub address: &'static str,
  /// Pool server
  pub host: &'static str,
  /// Pool port
  pub port: u16,
  /// Device name to show in the dashboard
  pub name: &'static str,
  /// Expected hashrate in kH/s
  pub hashrate: u32,
  // GPU devices to use
  pub devices: Vec<u32>,
  // Allocated memory in Mb for each device
  pub memory: Vec<u32>,
  //
  // Blockchain config
  //
  pub db_path: &'static str,
  pub db_size: usize,
  pub db_max: u32,
}

impl PoolConfig {
  #[inline]
  pub fn difficulty(&self) -> u32 {
    (1000 * self.hashrate * DESIRED_SPS) / (1 << 16)
  }

  // TODO: Propagate error
  #[inline]
  pub fn wsurl(&self) -> Url {
    Url::parse(&format!("wss://{}:{}", self.host, self.port)).unwrap()
  }
}

impl Default for PoolConfig {
  #[inline]
  fn default() -> Self {
    Self {
      address: "NQ33 G0T9 D63A TMN2 S18B 960Q S3Q3 TMYD KQJE",
      // host: "eu.sushipool.com",
      // host: "us.nimpool.io",
      host: "pool.nimiq.watch",
      // port: 443,
      // port: 8444,
      port: 8443,
      name: "My Miner", // os.hostname();
      hashrate: 100,    // 100 kH/s by default
      devices: vec![0],
      memory: vec![2048],

      db_path: "./db/",
      db_size: 1024 * 1024 * 50,
      db_max: 10,
    }
  }
}
