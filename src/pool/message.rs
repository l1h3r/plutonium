use std::collections::HashMap;

use crate::pool::PoolMode;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "message")]
#[serde(deny_unknown_fields)]
pub enum PoolMessage {
  /// Send by client directly after connecting to the server.
  ///
  /// https://nimiq-network.github.io/developer-reference/chapters/pool-protocol.html#register
  #[serde(rename = "register")]
  Register {
    /// The mode this client uses
    mode: PoolMode,
    /// The address (in IBAN-style format) that should be rewarded for the miner actions
    address: String,
    /// The clients device id. This ID should be unique for the device
    /// and stored, such that it stays the same after restarts
    #[serde(rename = "deviceId")]
    device: u32,
    /// A JSON object including stats about the device. The format
    /// of this JSON should be defined by the pool operator
    #[serde(rename = "deviceData")]
    data: Option<HashMap<String, String>>,
    /// Base64 encoded hash of the genesis block used by the client
    #[serde(rename = "genesisHash")]
    genesis: String,
  },
  /// Sent by the server after a succesful registration. Does not include parameters.
  ///
  /// https://nimiq-network.github.io/developer-reference/chapters/pool-protocol.html#registered
  #[serde(rename = "registered")]
  Registered,
  /// Sent by the server to announce new mining settings.
  ///
  /// https://nimiq-network.github.io/developer-reference/chapters/pool-protocol.html#settings
  #[serde(rename = "settings")]
  Settings {
    /// The address (in IBAN-style format) that the client should use as a miner address for future shares
    address: String,
    /// Base64 encoded buffer that the client should use in the extraData field for future shares
    #[serde(rename = "extraData")]
    extra: String,
    /// The maximum allowed hash value for future shares, in compact form
    #[serde(rename = "targetCompact")]
    target: u32,
    /// A number used once that is associated with this connection
    nonce: u64,
    /// ????
    #[serde(rename = "target")]
    _target: Option<String>,
  },
  /// Sent by the server to announce a new block that should be used by a client running in nano-mode.
  ///
  /// https://nimiq-network.github.io/developer-reference/chapters/pool-protocol.html#new-block-nano
  #[serde(rename = "new-block")]
  NewBlock {
    /// Base64 encoded hash of the block body
    #[serde(rename = "bodyHash")]
    body: String,
    /// Base64 encoded hash of the accounts tree after applying the block body
    #[serde(rename = "accountsHash")]
    accounts: String,
    /// Base64 encoded light block that is the predecessor of the block to be mined
    #[serde(rename = "previousBlock")]
    previous: String,
  },
  /// Sent by client when a valid share was found.
  ///
  /// https://nimiq-network.github.io/developer-reference/chapters/pool-protocol.html#share
  #[serde(rename = "share")]
  ShareNano {
    /// Base64 encoded light block
    block: String,
  },
  #[serde(rename = "share")]
  ShareSmart {
    /// Base64 encoded block header
    #[serde(rename = "blockHeader")]
    block_header: String,
    /// Base64 encoded inclusion proof for the minerAddr field in the block body
    #[serde(rename = "minerAddrProof")]
    miner_addr_proof: String,
    /// Base64 encoded inclusion proof for the extraData field in the block body
    #[serde(rename = "extraDataProof")]
    extra_data_proof: String,
    /// Base64 encoded full block. May only be sent if the block is
    /// a valid block so that the pool server is faster in picking it up
    block: Option<String>,
  },
  /// Sent by the server if the client sent an invalid share.
  /// The server may send this message at most once per share.
  ///
  /// https://nimiq-network.github.io/developer-reference/chapters/pool-protocol.html#error
  #[serde(rename = "error")]
  Error {
    /// A user-readable string explaining why the server denied the share
    reason: String,
  },
  /// Sent by the server to announce the balance that is currently held by the
  // pool for the address the user announced on register
  ///
  /// https://nimiq-network.github.io/developer-reference/chapters/pool-protocol.html#balance
  #[serde(rename = "balance")]
  Balance {
    /// The current balance of the user in the smallest possible unit. This includes funds that are not yet confirmed on the blockchain
    #[serde(rename = "balance")]
    balance: usize,
    /// The current balance of the user in the smallest possible unit. This only includes funds that operator considers confirmed and are available for payout
    #[serde(rename = "confirmedBalance")]
    confirmed: usize,
    /// true, if there is a payout request waiting for the user, false otherwise
    #[serde(rename = "payoutRequestActive")]
    payout: bool,
  },
  /// Sent by the client to request payout from the server. Depending on
  /// server configuration, manual payout may be suspect to an additional fee.
  ///
  /// https://nimiq-network.github.io/developer-reference/chapters/pool-protocol.html#payout
  #[serde(rename = "payout")]
  Payout {
    /// Base64 encoded signature proof of the string POOL_PAYOUT,
    /// concatenated with the byte representation of the connection nonce.
    proof: String,
  },
}
