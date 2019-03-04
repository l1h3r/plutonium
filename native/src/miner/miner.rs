use std::mem::size_of;
use std::os::raw::c_void;
use std::os::raw::c_char;
use std::ptr::null;

use crate::config::VENDOR_AMD;
use crate::config::VENDOR_NVIDIA;
use crate::config::INITIAL_SEED_SIZE;
use crate::config::ARGON2_BLOCK_SIZE;
use crate::config::ARGON2_MEMORY_COST;
use crate::config::NONCES_PER_GROUP;
use crate::config::THREADS_PER_LANE;
use crate::config::ONE_GB;
use crate::config::ONE_MB;
use crate::ffi::*;
use crate::hash::ARGON2D_CL;
use crate::hash::BLAKE2B_CL;
use crate::opencl::create_context;
use crate::opencl::create_buffer;
use crate::opencl::build_program;
use crate::opencl::create_command_queue;
use crate::opencl::create_program_with_source;
use crate::opencl::create_kernel;
use crate::opencl::get_platform_ids;
use crate::opencl::get_platform_info;
use crate::opencl::get_device_ids;
use crate::opencl::get_device_info;
use crate::opencl::set_kernel_arg;
use crate::error::Error;
use crate::miner::Worker;
use crate::utils::bytes_into;
use crate::utils::bytes_into_string;

#[derive(Debug)]
pub struct Miner {
  allowed_devices: Vec<u32>,
  memory_sizes: Vec<u32>,
  workers: Vec<Worker>,
}

impl Miner {
  pub fn new() -> Self {
    Self {
      allowed_devices: vec![],
      memory_sizes: vec![],
      workers: vec![],
    }
  }
}

impl Miner {
  fn seed() -> [u8; INITIAL_SEED_SIZE] {
    let mut seed = [0; INITIAL_SEED_SIZE];
    // TODO
    seed
  }

  #[inline]
  pub fn workers(&self) -> &[Worker] {
    &self.workers
  }

  #[inline]
  pub fn workers_mut(&mut self) -> &mut [Worker] {
    &mut self.workers
  }

  pub unsafe fn initialize(&mut self) -> Result<cl_int, Error> {
    self.workers = vec![];

    let platforms = get_platform_ids()?;

    println!("[+] Total Platforms: {}", platforms.len());
    println!("[+] Platforms: {:?}", platforms);

    // const char *sources[2] = {(char *)argon2d_cl, (char *)blake2b_cl};
    let sources: *const *const c_char = [
      ARGON2D_CL.as_ptr() as *const c_char,
      BLAKE2B_CL.as_ptr() as *const c_char,
    ].as_ptr();

    'platform: for platform in platforms.into_iter() {
      let name: String = get_platform_info(platform, CL_PLATFORM_NAME)?;
      let vendor: String = get_platform_info(platform, CL_PLATFORM_VENDOR)?;
      let is_amd: bool = vendor.starts_with(VENDOR_AMD);
      let is_nvidia: bool = vendor.starts_with(VENDOR_NVIDIA);

      println!("[+] Platform Name: {}", name);
      println!("[+] Platform Vendor: {}", vendor);

      if !is_amd && !is_nvidia {
        eprintln!("[x] Platform Not Supported");
        // continue 'platform;
      }

      let devices: Vec<cl_device_id> = get_device_ids(platform)?;

      if devices.is_empty() {
        eprintln!("[x] Cannot Find GPU Devices");
        continue 'platform;
      }

      println!("[+] Total Devices: {}", devices.len());
      println!("[+] Devices: {:?}", devices);

      // Iterate over devices, setup workers
      'device: for (index, device) in devices.into_iter().enumerate() {
        let gindex: cl_uint = self.workers.len() as cl_uint;

        // Check if this device is allowed
        if !self.is_allowed(index, gindex) {
          println!("[+] Device #{} disabled by user.", gindex);
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
        let driver_version: String = if driver_version.is_empty() { String::from("?") } else { driver_version };
        let device_version: String = if device_version.is_empty() { String::from("?") } else { device_version };

        // Calculate memory allocation
        let mut memory_size_mb: cl_ulong = 0;

        if !self.memory_sizes.is_empty() {
          if self.memory_sizes.len() == 1 {
            memory_size_mb = self.memory_sizes[0].into();
          } else if index < self.memory_sizes.len() {
            memory_size_mb = self.memory_sizes[index].into();
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
        let memory_cost: cl_uint = ARGON2_MEMORY_COST as cl_uint;
        let shmem_size: size_t = THREADS_PER_LANE * 2 * size_of::<cl_uint>() * jobs_per_block as size_t;

        println!("[+] Device #{}: {} by {}", gindex, device_name, device_vendor);
        println!("[+] Driver: {}, OpenCL: {}", driver_version, device_version);
        println!("[+] {} compute units @ {} MHz", max_compute_units, max_clock_frequency);
        println!("[+] Using {} MB of global memory, nonces per run: {}", memory_size_mb, nonces_per_run);

        println!("[+] Creating OpenCL Context: {:?}", device);

        let context: cl_context = create_context(device)?;

        println!("[+] OpenCL Context: {:?}", context);

        let blocks_mem_size: cl_ulong = (ARGON2_MEMORY_COST + if is_amd { 1 } else { 0 }) * ARGON2_BLOCK_SIZE * nonces_per_run;
        let blocks_mem_size: size_t = blocks_mem_size as size_t;

        println!("[+] Creating OpenCL Buffers: {:?}", context);

        println!("[+] Buffer Size (argon2): {:?}", blocks_mem_size);
        println!("[+] Buffer Size (seed): {:?}", INITIAL_SEED_SIZE);
        println!("[+] Buffer Size (nonce): {:?}", size_of::<cl_uint>());

        let mem_argon2_blocks: cl_mem = match create_buffer(context, blocks_mem_size) {
          Ok(buffer) => buffer,
          Err(error) => {
            eprintln!("{:?}", error);
            Err(Error::OpenCL(CL_MEM_OBJECT_ALLOCATION_FAILURE, "Failed to allocate required memory."))?
          }
        };

        let mem_initial_seed: cl_mem = create_buffer(context, INITIAL_SEED_SIZE)?;
        let mem_nonce: cl_mem = create_buffer(context, size_of::<cl_uint>())?;

        println!("[+] OpenCL Buffer (argon2): {:?}", mem_argon2_blocks);
        println!("[+] OpenCL Buffer (seed): {:?}", mem_initial_seed);
        println!("[+] OpenCL Buffer (nonce): {:?}", mem_nonce);

        // Cast kernel args to pointers
        let mem_initial_seed_ptr: *const c_void = &mem_initial_seed as *const *mut c_void as *const c_void;
        let memory_cost_ptr: *const c_void = &memory_cost as *const cl_uint as *const c_void;
        let mem_argon2_blocks_ptr: *const c_void = &mem_argon2_blocks as *const *mut c_void as *const c_void;
        let mem_nonce_ptr: *const c_void = &mem_nonce as *const *mut c_void as *const c_void;

        println!("[+] Creating OpenCL Program: {:?}/{:?}", context, device);

        let program: cl_program = create_program_with_source(context, sources)?;

        println!("[+] Building OpenCL program: {:?}", program);

        let _: () = build_program(program, device, if is_amd { "-Werror -DAMD" } else { "-Werror" })?;

        println!("[+] Creating OpenCL Command Queue: {:?}/{:?}", context, device);

        let queue = create_command_queue(context, device)?;

        println!("[+] Creating Kernel (init_memory)");

        let kernel_init_memory: cl_kernel = {
          let kernel = create_kernel(program, "init_memory\0" as *const str as *const c_char)?;
          let _: () = set_kernel_arg(kernel, 0, size_of::<cl_mem>(), mem_initial_seed_ptr)?;
          let _: () = set_kernel_arg(kernel, 1, size_of::<cl_mem>(), mem_argon2_blocks_ptr)?;
          let _: () = set_kernel_arg(kernel, 2, size_of::<cl_uint>(), memory_cost_ptr)?;
          kernel
        };

        println!("[+] Creating Kernel (argon2)");

        let kernel_argon2: cl_kernel = {
          let kernel = create_kernel(program, "argon2\0" as *const str as *const c_char)?;
          let _: () = set_kernel_arg(kernel, 0, shmem_size, null())?;
          let _: () = set_kernel_arg(kernel, 1, size_of::<cl_mem>(), mem_argon2_blocks_ptr)?;
          let _: () = set_kernel_arg(kernel, 2, size_of::<cl_uint>(), memory_cost_ptr)?;
          kernel
        };

        println!("[+] Creating Kernel (find_nonce)");

        let kernel_find_nonce: cl_kernel = {
          let kernel = create_kernel(program, "find_nonce\0" as *const str as *const c_char)?;
          // arg 0 is not available yet
          let _: () = set_kernel_arg(kernel, 1, size_of::<cl_mem>(), mem_argon2_blocks_ptr)?;
          let _: () = set_kernel_arg(kernel, 2, size_of::<cl_uint>(), memory_cost_ptr)?;
          let _: () = set_kernel_arg(kernel, 2, size_of::<cl_mem>(), mem_nonce_ptr)?;
          kernel
        };

        let worker: Worker = Worker {
          device_name,
          device_vendor,
          driver_version,
          device_version,
          max_compute_units,
          max_clock_frequency,
          max_mem_alloc_size,
          global_mem_size,
          nonces_per_run: nonces_per_run as cl_uint,
          device_index: gindex,
          device_id: device,
          context,
          queue,
          program,
          mem_initial_seed,
          mem_argon2_blocks,
          mem_nonce,
          kernel_init_memory,
          kernel_argon2,
          kernel_find_nonce,
          init_memory_global_size: [nonces_per_run as size_t, jobs_per_block as size_t],
          init_memory_local_size: [NONCES_PER_GROUP, jobs_per_block as size_t],
          argon2_global_size: [THREADS_PER_LANE, nonces_per_run as size_t],
          argon2_local_size: [THREADS_PER_LANE, jobs_per_block as size_t],
          find_nonce_global_size: [nonces_per_run as size_t],
          find_nonce_local_size: [NONCES_PER_GROUP],
          ..Worker::new()
        };

        println!("\nWorker: {:#?}", worker);

        self.workers.push(worker);
      }
    }

    if self.workers.is_empty() {
      Err(Error::OpenCL(CL_DEVICE_NOT_FOUND, "Failed to find any usable GPU devices."))?
    }

    Ok(CL_SUCCESS)
  }

  pub unsafe fn release(&mut self) -> Result<cl_int, Error> {
    for worker in &mut self.workers {
      let _ = worker.release()?;
    }

    Ok(CL_SUCCESS)
  }

  fn is_allowed(&self, index: usize, gindex: cl_uint) -> bool {
    if self.allowed_devices.is_empty() {
      return true;
    }

    self
      .allowed_devices
      .get(index)
      .map(|&device| device == gindex)
      .unwrap_or(false)
  }
}
