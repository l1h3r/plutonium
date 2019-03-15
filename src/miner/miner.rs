use base64::encode;
use beserial::Serialize;
use byteorder::ByteOrder;
use byteorder::LittleEndian;
use nimiq_block::Block;
use nimiq_block::BlockHeader;
use nimiq_blockchain::Blockchain;
use nimiq_hash::Argon2dHash;
use nimiq_hash::Blake2bHash;
use nimiq_hash::Hash;
use std::mem::size_of;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::ptr::null;
use std::sync::Arc;
use std::sync::RwLock;

use crate::config::ARGON2_BLOCK_SIZE;
use crate::config::ARGON2_HASH_LENGTH;
use crate::config::ARGON2_ITERATIONS;
use crate::config::ARGON2_LANES;
use crate::config::ARGON2_MEMORY_COST as _ARGON2_MEMORY_COST;
use crate::config::ARGON2_SALT;
use crate::config::ARGON2_TYPE;
use crate::config::ARGON2_VERSION;
use crate::config::INITIAL_SEED_SIZE;
use crate::config::NONCES_PER_GROUP;
use crate::config::ONE_GB;
use crate::config::ONE_MB;
use crate::config::THREADS_PER_LANE;
use crate::config::VENDOR_AMD;
use crate::config::VENDOR_NVIDIA;
use crate::error::Error;
use crate::ffi::*;
use crate::hash::Source;
use crate::hash::ARGON2D_CL;
use crate::hash::BLAKE2B_CL;
use crate::miner::MinerConfig;
use crate::miner::Worker;
use crate::opencl::build_program;
use crate::opencl::create_buffer;
use crate::opencl::create_command_queue;
use crate::opencl::create_context;
use crate::opencl::create_kernel;
use crate::opencl::create_program_with_source;
use crate::opencl::get_device_ids;
use crate::opencl::get_device_info;
use crate::opencl::get_platform_ids;
use crate::opencl::get_platform_info;
use crate::opencl::set_kernel_arg;
use crate::pool::PoolMessage;
use crate::utils::bytes_into;
use crate::utils::bytes_into_string;

type Seed = [u8; INITIAL_SEED_SIZE];
type ArcChain = Arc<Blockchain<'static>>;

static ARGON2_MEMORY_COST: u64 = _ARGON2_MEMORY_COST as u64;
static BLOCK_HEADER_LEN: u32 = 146;
static MAX_NONCE: usize = 4294967296; // 2 ** 32

lazy_static! {
  pub static ref MINER: Arc<RwLock<Miner>> = {
    let mut miner = Miner::new();

    miner.initialize().unwrap();

    Arc::new(RwLock::new(miner))
  };
  static ref SEED: Seed = {
    let mut seed: Seed = [0; INITIAL_SEED_SIZE];
    let salt: &[u8] = ARGON2_SALT.as_bytes();

    LittleEndian::write_u32(&mut seed[0..4], ARGON2_LANES);
    LittleEndian::write_u32(&mut seed[4..8], ARGON2_HASH_LENGTH);
    LittleEndian::write_u32(&mut seed[8..12], _ARGON2_MEMORY_COST);
    LittleEndian::write_u32(&mut seed[12..16], ARGON2_ITERATIONS);
    LittleEndian::write_u32(&mut seed[16..204], ARGON2_VERSION);
    LittleEndian::write_u32(&mut seed[20..24], ARGON2_TYPE);
    LittleEndian::write_u32(&mut seed[24..28], BLOCK_HEADER_LEN);
    LittleEndian::write_u32(&mut seed[174..178], salt.len() as u32);

    seed[178..(178 + salt.len())].copy_from_slice(salt);

    seed
  };
  static ref SOURCES: [Source; 2] = { [Source::new(ARGON2D_CL), Source::new(BLAKE2B_CL)] };
}

// #[derive(Debug)]
#[repr(C)]
pub struct Miner {
  nonce: cl_uint,
  workid: cl_uint,
  scompact: cl_uint,
  zero: cl_uint,
  hashcount: usize,
  // hashrate: usize,
  config: MinerConfig,
  workers: Vec<Worker>,
  seed: [u8; INITIAL_SEED_SIZE],
}

//
// Warning: BYO Safety
//
unsafe impl Send for Miner {}
unsafe impl Sync for Miner {}

impl Miner {
  pub fn new() -> Self {
    Self {
      config: MinerConfig {
        devices: vec![],
        memsizes: vec![],
      },
      nonce: 0,
      workid: 0,
      scompact: 0,
      zero: 0,
      hashcount: 0,
      // hashrate: 0,
      workers: vec![],
      seed: [0; INITIAL_SEED_SIZE],
    }
  }

  pub fn mine(&mut self, block: Block, blockchain: ArcChain) -> Result<(), Error> {
    self.workid += 1;
    self.nonce = 0;
    self.seed = self._seed(&block.header);

    for worker in &self.workers {
      let nonces = worker.nonces_per_run;
      let workid = self.workid;

      unsafe {
        worker.setup(self.seed_ptr(), self.zero_ptr())?;

        'mine: loop {
          let nonce = self.nonce;

          self.nonce += nonces;

          let nnonce = worker.mine(&nonce, &self.scompact, self.zero_ptr())?;

          // Another block arrived
          if workid != self.workid {
            break 'mine;
          }

          if nnonce > 0 {
            // println!("Found Share: {} - {:?}", nnonce, block);

            self.hashcount += 1;

            let head_hash: Blake2bHash = blockchain.head_hash();

            if block.header.prev_hash == head_hash {
              let mut block = block.clone();

              block.header.nonce = nnonce;

              let hash: Argon2dHash = block.header.pow();

              println!("[+] Received Share: {} - {}", nnonce, hash.to_hex());

              if block.body.is_some() {
                if block.header.verify_proof_of_work() {
                  println!("Block Valid: {}", block.header.pow());
                } else {
                  println!("Invalid Block POF: {}", block.header.pow());
                }
              } else {
                println!("[+] Invalid Block Body: {}", block.header.pow());
              }
            // if (!this._submittingBlock) {
            //   block.header.nonce = nnonce;

            //   let blockValid = false;
            //   if block.isFull() && BlockUtils.isProofOfWork(hash, block.target) {
            //     this._submittingBlock = true;
            //     if (await block.header.verifyProofOfWork()) {
            //       this._numBlocksMined++;
            //       blockValid = true;

            //       // Tell listeners that we've mined a block.
            //       this.fire('block-mined', block, this);

            //       // Push block into blockchain.
            //       if ((await this._blockchain.pushBlock(block)) < 0) {
            //         this._submittingBlock = false;
            //         this._startWork().catch(Log.w.tag(Miner));
            //         return;
            //       } else {
            //         this._submittingBlock = false;
            //       }
            //     } else {
            //       Log.d(Miner, ``);
            //     }
            //   }

            //   this.fire('share', block, blockValid, this);
            // }

            // let share = PoolMessage::ShareNano {
            //   block: encode(&block.serialize_to_vec()),
            // };

            // println!("Pool Share: {:?}", share);

            // this._send({
            //   message: 'share',
            //   block: BufferUtils.toBase64(block.serialize())
            // });
            } else {
              println!("[x] Invalid Share: {}/{}", nnonce, block.header.pow());
            }
          }

          if self.nonce as usize >= MAX_NONCE {
            break 'mine;
          }
        }
      }
    }

    Ok(())
  }

  #[inline]
  pub fn workers(&self) -> &[Worker] {
    &self.workers
  }

  #[inline]
  pub fn workers_mut(&mut self) -> &mut [Worker] {
    &mut self.workers
  }

  #[inline]
  pub fn initialize(&mut self) -> Result<(), Error> {
    self.workers = unsafe { initialize(&self.config)? };

    Ok(())
  }

  #[inline]
  pub unsafe fn release(&mut self) -> Result<cl_int, Error> {
    for worker in &mut self.workers {
      let _ = worker.release()?;
    }

    Ok(CL_SUCCESS)
  }

  #[inline]
  pub fn scompact(&mut self, scompact: u32) {
    self.scompact = scompact;
  }

  #[inline]
  fn seed_ptr(&self) -> *const c_void {
    self.seed.as_ptr() as *const c_void
  }

  #[inline]
  fn zero_ptr(&self) -> *const c_void {
    (&self.zero) as *const cl_uint as *const c_void
  }

  fn _seed(&self, header: &BlockHeader) -> Seed {
    let mut seed: Seed = SEED.clone();
    let serialized: Vec<u8> = header.serialize_to_vec();
    let header_len: usize = BLOCK_HEADER_LEN as usize;

    if serialized.len() != header_len {
      panic!("Invalid Block Serialization: {}/{}", serialized.len(), BLOCK_HEADER_LEN);
    }

    seed[28..(28 + header_len)].copy_from_slice(&serialized);

    seed
  }
}

impl std::fmt::Debug for Miner {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    f.debug_struct("Miner")
      .field("nonce", &self.nonce)
      .field("work id", &self.workid)
      .field("config", &self.config)
      .field("workers", &self.workers)
      .field("share compact", &self.scompact)
      .finish()
  }
}

unsafe fn initialize(config: &MinerConfig) -> Result<Vec<Worker>, Error> {
  // Find all OpenCL platforms
  let platforms: Vec<cl_platform_id> = get_platform_ids()?;

  println!("[+] Platforms");
  println!("[+] - Total    = {}", platforms.len());
  println!("[+] - Pointers = {:?}", platforms);

  let mut workers: Vec<Worker> = vec![];

  'platform: for platform in platforms.into_iter() {
    let name: String = get_platform_info(platform, CL_PLATFORM_NAME)?;
    let vendor: String = get_platform_info(platform, CL_PLATFORM_VENDOR)?;
    let is_amd: bool = vendor.starts_with(VENDOR_AMD);
    let is_nvidia: bool = vendor.starts_with(VENDOR_NVIDIA);

    println!("[+] Platform");
    println!("[+] - Name   = {}", name);
    println!("[+] - Vendor = {}", vendor);
    println!("[+] - AMD    = {}", is_amd);
    println!("[+] - Nvidia = {}", is_nvidia);

    if !is_amd && !is_nvidia {
      eprintln!("[x] Unsupported Platform: {}/{}", name, vendor);
      // continue 'platform;
    }

    // Find all GPU devices
    let devices: Vec<cl_device_id> = get_device_ids(platform)?;

    if devices.is_empty() {
      eprintln!("[x] No GPU Devices Found");
      continue 'platform;
    }

    println!("[+] Devices");
    println!("[+] - Total    = {}", devices.len());
    println!("[+] - Pointers = {:?}", devices);

    // Iterate over devices, setup workers
    'device: for (index, device) in devices.into_iter().enumerate() {
      let gindex: cl_uint = workers.len() as cl_uint + 1;

      // Check if this device is allowed
      if !config.allowed_device(index, gindex) {
        println!("[+] Device #{} Disabled", gindex);
        continue 'device;
      }

      // Fetch device info
      let device_name: Vec<u8> = get_device_info(device, CL_DEVICE_NAME)?;
      let device_vendor: Vec<u8> = get_device_info(device, CL_DEVICE_VENDOR)?;
      let driver_version: Vec<u8> = get_device_info(device, CL_DRIVER_VERSION)?;
      let device_version: Vec<u8> = get_device_info(device, CL_DEVICE_VERSION)?;
      let max_compute_units: Vec<u8> = get_device_info(device, CL_DEVICE_MAX_COMPUTE_UNITS)?;
      let max_clock_frequency: Vec<u8> = get_device_info(device, CL_DEVICE_MAX_CLOCK_FREQUENCY)?;
      let max_mem_alloc_size: Vec<u8> = get_device_info(device, CL_DEVICE_MAX_MEM_ALLOC_SIZE)?;
      let global_mem_size: Vec<u8> = get_device_info(device, CL_DEVICE_GLOBAL_MEM_SIZE)?;

      // Cast device info to proper types
      let device_name: String = bytes_into_string(device_name); // TODO: Limit to 255 chars?
      let device_vendor: String = bytes_into_string(device_vendor); // TODO: Limit to 255 chars?
      let driver_version: String = bytes_into_string(driver_version); // TODO: Limit to 64 chars?
      let device_version: String = bytes_into_string(device_version); // TODO: Limit to 64 chars?
      let max_compute_units: cl_uint = bytes_into(max_compute_units)?;
      let max_clock_frequency: cl_uint = bytes_into(max_clock_frequency)?;
      let max_mem_alloc_size: cl_ulong = bytes_into(max_mem_alloc_size)?;
      let global_mem_size: cl_ulong = bytes_into(global_mem_size)?;

      // Set default driver/device versions (unknown)
      let driver_version: String = if driver_version.is_empty() {
        String::from("?")
      } else {
        driver_version
      };

      let device_version: String = if device_version.is_empty() {
        String::from("?")
      } else {
        device_version
      };

      // Calculate memory allocation
      let mut memory_size_mb: cl_ulong = 0;

      if !config.memsizes.is_empty() {
        if config.memsizes.len() == 1 {
          memory_size_mb = config.memsizes[0].into();
        } else if index < config.memsizes.len() {
          memory_size_mb = config.memsizes[index].into();
        }
      }

      if memory_size_mb == 0 {
        let memory_size_gb: cl_ulong = if is_amd {
          max_mem_alloc_size
        } else {
          global_mem_size / 2
        };

        memory_size_mb = ((memory_size_gb as f64 / ONE_GB) * 1024.0) as cl_ulong;
      }

      let nonces_per_run: cl_ulong = (memory_size_mb * ONE_MB) / (ARGON2_BLOCK_SIZE * ARGON2_MEMORY_COST);
      let jobs_per_block: cl_uint = if is_amd { 2 } else { 1 };
      let shmem_size: size_t = THREADS_PER_LANE * 2 * size_of::<cl_uint>() * jobs_per_block as size_t;
      let blocks_mem_size: cl_ulong =
        (ARGON2_MEMORY_COST + if is_amd { 1 } else { 0 }) * ARGON2_BLOCK_SIZE * nonces_per_run;
      let blocks_mem_size: size_t = blocks_mem_size as size_t;

      println!("[+] Device #{}", gindex);
      println!("[+] - Name           = {}", device_name);
      println!("[+] - Vendor         = {}", device_vendor);
      println!("[+] - Driver         = {}", driver_version);
      println!("[+] - OpenCL         = {}", device_version);
      println!("[+] - Jobs Per Block = {}", jobs_per_block);
      println!(
        "[+] - {} Compute Units @ {} MHz",
        max_compute_units, max_clock_frequency
      );
      println!(
        "[+] - Using {} MB of Global Memory, Nonces Per Run: {}",
        memory_size_mb, nonces_per_run
      );

      let mut worker: Worker = Worker {
        device_name,
        device_vendor,
        driver_version,
        device_version,
        max_compute_units,
        max_clock_frequency,
        max_mem_alloc_size,
        global_mem_size,
        device_index: gindex,
        device_id: device,
        nonces_per_run: nonces_per_run as cl_uint,
        init_memory_global_size: [nonces_per_run as size_t, jobs_per_block as size_t],
        init_memory_local_size: [NONCES_PER_GROUP, jobs_per_block as size_t],
        argon2_global_size: [THREADS_PER_LANE, nonces_per_run as size_t],
        argon2_local_size: [THREADS_PER_LANE, jobs_per_block as size_t],
        find_nonce_global_size: [nonces_per_run as size_t],
        find_nonce_local_size: [NONCES_PER_GROUP],
        ..Worker::new()
      };

      println!("[+] Creating OpenCL Context");

      worker.context = create_context(device)?;

      println!("[+] Creating OpenCL Buffers");

      worker.mem_argon2_blocks = create_buffer(worker.context, blocks_mem_size)?;
      worker.mem_initial_seed = create_buffer(worker.context, INITIAL_SEED_SIZE)?;
      worker.mem_nonce = create_buffer(worker.context, size_of::<cl_uint>())?;

      println!("[+] Creating OpenCL Program");

      // Cast kernel/program args to pointers
      let mem_initial_seed_ptr: *const c_void = &worker.mem_initial_seed as *const *mut c_void as *const c_void;
      let memory_cost_ptr: *const c_void = &_ARGON2_MEMORY_COST as *const cl_uint as *const c_void;
      let mem_argon2_blocks_ptr: *const c_void = &worker.mem_argon2_blocks as *const *mut c_void as *const c_void;
      let mem_nonce_ptr: *const c_void = &worker.mem_nonce as *const *mut c_void as *const c_void;
      let program_sources: *const *const c_char = SOURCES.as_ptr() as *const *const c_char;

      worker.program = create_program_with_source(worker.context, program_sources)?;

      println!("[+] Building OpenCL program");

      let _: () = build_program(worker.program, device, if is_amd { "-Werror -DAMD" } else { "-Werror" })?;

      println!("[+] Creating OpenCL Command Queue");

      worker.queue = create_command_queue(worker.context, device)?;

      println!("[+] Creating OpenCL Kernel (init)");

      worker.kernel_init_memory = {
        let kernel = create_kernel(worker.program, "init_memory\0" as *const str as *const c_char)?;
        let _: () = set_kernel_arg(kernel, 0, size_of::<cl_mem>(), mem_initial_seed_ptr)?;
        let _: () = set_kernel_arg(kernel, 1, size_of::<cl_mem>(), mem_argon2_blocks_ptr)?;
        let _: () = set_kernel_arg(kernel, 2, size_of::<cl_uint>(), memory_cost_ptr)?;
        kernel
      };

      println!("[+] Creating OpenCL Kernel (argon2)");

      worker.kernel_argon2 = {
        let kernel = create_kernel(worker.program, "argon2\0" as *const str as *const c_char)?;
        let _: () = set_kernel_arg(kernel, 0, shmem_size, null())?;
        let _: () = set_kernel_arg(kernel, 1, size_of::<cl_mem>(), mem_argon2_blocks_ptr)?;
        let _: () = set_kernel_arg(kernel, 2, size_of::<cl_uint>(), memory_cost_ptr)?;
        kernel
      };

      println!("[+] Creating OpenCL Kernel (nonce)");

      worker.kernel_find_nonce = {
        let kernel = create_kernel(worker.program, "find_nonce\0" as *const str as *const c_char)?;
        // arg 0 is not available yet
        let _: () = set_kernel_arg(kernel, 1, size_of::<cl_mem>(), mem_argon2_blocks_ptr)?;
        let _: () = set_kernel_arg(kernel, 2, size_of::<cl_uint>(), memory_cost_ptr)?;
        let _: () = set_kernel_arg(kernel, 3, size_of::<cl_mem>(), mem_nonce_ptr)?;
        kernel
      };

      workers.push(worker);
    }
  }

  if workers.is_empty() {
    Err(Error::OpenCL(
      CL_DEVICE_NOT_FOUND,
      "Failed to find any usable GPU devices.",
    ))?
  }

  Ok(workers)
}
