use std::mem::size_of;
use std::mem::transmute;
use std::os::raw::c_void;
use std::ptr::null;
use std::ptr::null_mut;

use crate::config::INITIAL_SEED_SIZE;
use crate::error::Error;
use crate::ffi::clEnqueueNDRangeKernel;
use crate::ffi::clEnqueueReadBuffer;
use crate::ffi::cl_command_queue;
use crate::ffi::cl_context;
use crate::ffi::cl_device_id;
use crate::ffi::cl_int;
use crate::ffi::cl_kernel;
use crate::ffi::cl_mem;
use crate::ffi::cl_program;
use crate::ffi::cl_uint;
use crate::ffi::cl_ulong;
use crate::ffi::size_t;
use crate::ffi::CL_FALSE;
use crate::ffi::CL_SUCCESS;
use crate::ffi::CL_TRUE;
use crate::opencl::enqueue_write_buffer;
use crate::opencl::release_command_queue;
use crate::opencl::release_context;
use crate::opencl::release_kernel;
use crate::opencl::release_mem_object;
use crate::opencl::release_program;
use crate::opencl::set_kernel_arg;

pub static ZERO: cl_uint = 0;

#[derive(Debug)]
#[repr(C)]
pub struct Worker {
  pub device_name: String,    // [c_char; 255],
  pub device_vendor: String,  // [c_char; 255],
  pub driver_version: String, // [c_char; 64],
  pub device_version: String, // [c_char; 64],
  pub max_compute_units: cl_uint,
  pub max_clock_frequency: cl_uint,
  pub max_mem_alloc_size: cl_ulong,
  pub global_mem_size: cl_ulong,
  pub nonces_per_run: cl_uint,
  pub device_index: cl_uint,
  pub device_id: cl_device_id,
  pub context: cl_context,
  pub queue: cl_command_queue,
  pub program: cl_program,
  pub mem_initial_seed: cl_mem,
  pub mem_argon2_blocks: cl_mem,
  pub mem_nonce: cl_mem,
  pub kernel_init_memory: cl_kernel,
  pub kernel_argon2: cl_kernel,
  pub kernel_find_nonce: cl_kernel,
  pub init_memory_global_size: [size_t; 2],
  pub init_memory_local_size: [size_t; 2],
  pub argon2_global_size: [size_t; 2],
  pub argon2_local_size: [size_t; 2],
  pub find_nonce_global_size: [size_t; 1],
  pub find_nonce_local_size: [size_t; 1],
}

//
// Warning: BYO Safety
//
unsafe impl Send for Worker {}
unsafe impl Sync for Worker {}

impl Worker {
  pub fn new() -> Self {
    Self {
      device_name: String::from("?"),    // [0; 255],
      device_vendor: String::from("?"),  // [0; 255],
      driver_version: String::from("?"), // [0; 64],
      device_version: String::from("?"), // [0; 64],
      max_compute_units: 0,
      max_clock_frequency: 0,
      max_mem_alloc_size: 0,
      global_mem_size: 0,
      nonces_per_run: 0,
      device_index: 0,
      device_id: null_mut(),
      context: null_mut(),
      queue: null_mut(),
      program: null_mut(),
      mem_initial_seed: null_mut(),
      mem_argon2_blocks: null_mut(),
      mem_nonce: null_mut(),
      kernel_init_memory: null_mut(),
      kernel_argon2: null_mut(),
      kernel_find_nonce: null_mut(),
      init_memory_global_size: [0; 2],
      init_memory_local_size: [0; 2],
      argon2_global_size: [0; 2],
      argon2_local_size: [0; 2],
      find_nonce_global_size: [0; 1],
      find_nonce_local_size: [0; 1],
    }
  }

  pub unsafe fn setup(&self, seed: *const c_void, zero: *const c_void) -> Result<cl_int, Error> {
    enqueue_write_buffer(self.queue, self.mem_initial_seed, CL_FALSE, INITIAL_SEED_SIZE, seed)?;

    enqueue_write_buffer(self.queue, self.mem_nonce, CL_TRUE, size_of::<cl_uint>(), zero)?;

    Ok(CL_SUCCESS)
  }

  pub unsafe fn mine(
    &self,
    nonce: *const cl_uint,
    scompact: *const cl_uint,
    zero: *const c_void,
  ) -> Result<cl_uint, Error> {
    // Initialize memory
    let result: cl_int = clEnqueueNDRangeKernel(
      self.queue,
      self.kernel_init_memory,
      2,
      [*nonce as size_t, 0].as_ptr(),
      self.init_memory_global_size.as_ptr(),
      self.init_memory_local_size.as_ptr(),
      0,
      null(),
      null_mut(),
    );

    if result != CL_SUCCESS {
      Err(Error::OpenCL(result, "clEnqueueNDRangeKernel"))?
    }

    // Compute Argon2d hashes
    let result: cl_int = clEnqueueNDRangeKernel(
      self.queue,
      self.kernel_argon2,
      2,
      null(),
      self.argon2_global_size.as_ptr(),
      self.argon2_local_size.as_ptr(),
      0,
      null(),
      null_mut(),
    );

    if result != CL_SUCCESS {
      Err(Error::OpenCL(result, "clEnqueueNDRangeKernel"))?
    }

    // Is there PoW?
    let _: () = set_kernel_arg(
      self.kernel_find_nonce,
      0,
      size_of::<cl_uint>(),
      scompact as *const c_void,
    )?;

    let result: cl_int = clEnqueueNDRangeKernel(
      self.queue,
      self.kernel_find_nonce,
      1,
      nonce as *const size_t,
      self.find_nonce_global_size.as_ptr(),
      self.find_nonce_local_size.as_ptr(),
      0,
      null(),
      null_mut(),
    );

    if result != CL_SUCCESS {
      Err(Error::OpenCL(result, "clEnqueueNDRangeKernel 3"))?
    }

    let mut next: [u8; 4] = [0; 4];

    let result: cl_int = clEnqueueReadBuffer(
      self.queue,
      self.mem_nonce,
      CL_TRUE,
      0,
      size_of::<cl_uint>(),
      next.as_mut_ptr() as *mut c_void,
      0,
      null(),
      null_mut(),
    );

    if result != CL_SUCCESS {
      Err(Error::OpenCL(result, "clEnqueueReadBuffer"))?
    }

    let next: cl_uint = transmute(next);

    if next > 0 {
      enqueue_write_buffer(self.queue, self.mem_nonce, CL_TRUE, size_of::<cl_uint>(), zero)?;
    }

    Ok(next)
  }

  pub unsafe fn release(&mut self) -> Result<cl_int, Error> {
    release_kernel(self.kernel_init_memory)?;
    release_kernel(self.kernel_argon2)?;
    release_kernel(self.kernel_find_nonce)?;
    release_mem_object(self.mem_initial_seed)?;
    release_mem_object(self.mem_argon2_blocks)?;
    release_mem_object(self.mem_nonce)?;
    release_program(self.program)?;
    release_command_queue(self.queue)?;
    release_context(self.context)?;

    Ok(CL_SUCCESS)
  }
}
