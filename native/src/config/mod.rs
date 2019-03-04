pub const INITIAL_SEED_SIZE: usize = 256;
pub const ARGON2_BLOCK_SIZE: u64 = 1024;
pub const ARGON2_MEMORY_COST: u64 = 512;

pub const VENDOR_AMD: &'static str = "Advanced Micro Devices";
pub const VENDOR_NVIDIA: &'static str = "NVIDIA Corporation";

pub const ONE_GB: f64 = 0x40000000 as f64;
pub const ONE_MB: u64 = 0x100000;

pub const NONCES_PER_GROUP: usize = 32;
pub const THREADS_PER_LANE: usize = 32;
