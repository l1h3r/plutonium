pub const INITIAL_SEED_SIZE: usize = 256;

pub static ARGON2_BLOCK_SIZE: u64 = 1024;

pub static VENDOR_AMD: &'static str = "Advanced Micro Devices";
pub static VENDOR_NVIDIA: &'static str = "NVIDIA Corporation";

pub static ONE_GB: f64 = 0x40000000 as f64;
pub static ONE_MB: u64 = 0x100000;

pub static NONCES_PER_GROUP: usize = 32;
pub static THREADS_PER_LANE: usize = 32;

pub static ARGON2_ITERATIONS: u32 = 1;
pub static ARGON2_LANES: u32 = 1;
pub static ARGON2_MEMORY_COST: u32 = 512;
pub static ARGON2_VERSION: u32 = 0x13;
pub static ARGON2_TYPE: u32 = 0; // Argon2D
pub static ARGON2_SALT: &'static str = "nimiqrocks!";
pub static ARGON2_HASH_LENGTH: u32 = 32;
