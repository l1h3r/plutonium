pub mod config;
pub mod error;
pub mod ffi;
pub mod hash;
pub mod miner;
pub mod opencl;
pub mod utils;

use crate::miner::Miner;

#[no_mangle]
pub extern "C" fn miner() -> Miner {
  Miner::new()
}

#[no_mangle]
pub extern "C" fn test() -> i32 {
  0
}
