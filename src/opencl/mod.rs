use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::ptr::null;
use std::ptr::null_mut;

use crate::ffi::CL_BUILD_PROGRAM_FAILURE;
use crate::ffi::CL_DEVICE_TYPE_GPU;
use crate::ffi::CL_INVALID_PLATFORM;
use crate::ffi::CL_INVALID_VALUE;
use crate::ffi::CL_MEM_READ_WRITE;
use crate::ffi::CL_PROGRAM_BUILD_LOG;
use crate::ffi::CL_SUCCESS;

use crate::ffi::cl_bool;
use crate::ffi::cl_command_queue;
use crate::ffi::cl_context;
use crate::ffi::cl_device_id;
use crate::ffi::cl_device_info;
use crate::ffi::cl_int;
use crate::ffi::cl_kernel;
use crate::ffi::cl_mem;
use crate::ffi::cl_platform_id;
use crate::ffi::cl_platform_info;
use crate::ffi::cl_program;
use crate::ffi::cl_uint;
use crate::ffi::size_t;

use crate::ffi::clBuildProgram;
use crate::ffi::clCreateBuffer;
use crate::ffi::clCreateCommandQueue;
use crate::ffi::clCreateContext;
use crate::ffi::clCreateKernel;
use crate::ffi::clCreateProgramWithSource;
use crate::ffi::clEnqueueWriteBuffer;
use crate::ffi::clGetDeviceIDs;
use crate::ffi::clGetDeviceInfo;
use crate::ffi::clGetPlatformIDs;
use crate::ffi::clGetPlatformInfo;
use crate::ffi::clGetProgramBuildInfo;
use crate::ffi::clReleaseCommandQueue;
use crate::ffi::clReleaseContext;
use crate::ffi::clReleaseKernel;
use crate::ffi::clReleaseMemObject;
use crate::ffi::clReleaseProgram;
use crate::ffi::clSetKernelArg;

use crate::error::Error;
use crate::utils::bytes_into_string;

#[inline]
pub unsafe fn create_kernel(program: cl_program, name: *const c_char) -> Result<cl_kernel, Error> {
  let mut error: cl_int = CL_INVALID_VALUE;
  let kernel: cl_kernel = clCreateKernel(program, name, &mut error);

  if kernel.is_null() {
    Err("Cannot Create Kernel")?
  }

  if error == CL_SUCCESS {
    Ok(kernel)
  } else {
    Err(Error::OpenCL(error, "clCreateKernel"))?
  }
}

#[inline]
pub unsafe fn create_context(device: cl_device_id) -> Result<cl_context, Error> {
  let mut error: cl_int = CL_INVALID_VALUE;
  let context: cl_context = clCreateContext(null(), 1, &device, None, null_mut(), &mut error);

  if context.is_null() {
    Err("Cannot Create Context")?
  }

  if error == CL_SUCCESS {
    Ok(context)
  } else {
    Err(Error::OpenCL(error, "clCreateContext"))?
  }
}

#[inline]
pub unsafe fn create_buffer(context: cl_context, size: size_t) -> Result<cl_mem, Error> {
  let mut error: cl_int = CL_INVALID_VALUE;
  let buffer: cl_mem = clCreateBuffer(context, CL_MEM_READ_WRITE, size, null_mut(), &mut error);

  if buffer.is_null() {
    Err("Cannot Create Buffer")?
  }

  if error == CL_SUCCESS {
    Ok(buffer)
  } else {
    Err(Error::OpenCL(error, "clCreateBuffer"))?
  }
}

#[inline]
pub unsafe fn create_command_queue(context: cl_context, device: cl_device_id) -> Result<cl_command_queue, Error> {
  let mut error: cl_int = CL_INVALID_VALUE;
  let queue: cl_command_queue = clCreateCommandQueue(context, device, 0, &mut error);

  if queue.is_null() {
    Err("Cannot Create Queue")?
  }

  if error == CL_SUCCESS {
    Ok(queue)
  } else {
    Err(Error::OpenCL(error, "clCreateCommandQueue"))?
  }
}

#[inline]
pub unsafe fn create_program_with_source(
  context: cl_context,
  strings: *const *const c_char,
) -> Result<cl_program, Error> {
  let mut error: cl_int = CL_INVALID_VALUE;
  let program: cl_program = clCreateProgramWithSource(context, 2, strings, null(), &mut error);

  if program.is_null() {
    Err("Cannot Create Program")?
  }

  if error == CL_SUCCESS {
    Ok(program)
  } else {
    Err(Error::OpenCL(error, "clCreateProgramWithSource"))?
  }
}

#[inline]
pub unsafe fn build_program(program: cl_program, device: cl_device_id, options: &'static str) -> Result<(), Error> {
  let options_ptr: *const c_char = CString::new(options).unwrap().as_ptr();
  let result: cl_int = clBuildProgram(program, 0, null(), options_ptr, None, null_mut());

  if result != CL_SUCCESS {
    let info: Vec<u8> = get_program_build_info(program, device)?;
    let info: String = bytes_into_string(info);

    eprintln!("Failed to build program: {}", info);

    Err(Error::OpenCL(CL_BUILD_PROGRAM_FAILURE, "Failed to build program."))?
  }

  Ok(())
}

#[inline]
pub unsafe fn get_platform_ids() -> Result<Vec<cl_platform_id>, Error> {
  let mut count: cl_uint = 0;

  let result: cl_int = clGetPlatformIDs(0, null_mut(), &mut count);

  if result != CL_SUCCESS {
    Err(Error::OpenCL(result, "clGetPlatformIDs"))?
  }

  if count == 0 {
    Err(Error::OpenCL(CL_INVALID_PLATFORM, "clGetPlatformIDs"))?
  }

  let mut platforms: Vec<cl_platform_id> = vec![null_mut(); count as usize];

  let result: cl_int = clGetPlatformIDs(count, platforms.as_mut_ptr() as *mut cl_platform_id, null_mut());

  if result != CL_SUCCESS {
    Err(Error::OpenCL(result, "clGetPlatformIDs"))?
  }

  Ok(platforms)
}

#[inline]
pub unsafe fn get_platform_info(platform: cl_platform_id, info: cl_platform_info) -> Result<String, Error> {
  let mut size: size_t = 0;

  let result: cl_int = clGetPlatformInfo(platform, info, 0, null_mut(), &mut size);

  if result != CL_SUCCESS {
    Err(Error::OpenCL(result, "clGetPlatformInfo"))?
  }

  if size == 0 {
    return Ok(String::new());
  }

  let mut buffer: Vec<u8> = vec![0; size];

  let result: cl_int = clGetPlatformInfo(platform, info, size, buffer.as_mut_ptr() as *mut c_void, null_mut());

  if result != CL_SUCCESS {
    Err(Error::OpenCL(result, "clGetPlatformInfo"))?
  }

  Ok(bytes_into_string(buffer))
}

#[inline]
pub unsafe fn get_device_ids(platform: cl_platform_id) -> Result<Vec<cl_device_id>, Error> {
  let mut count: cl_uint = 0;

  let result: cl_int = clGetDeviceIDs(platform, CL_DEVICE_TYPE_GPU, 0, null_mut(), &mut count);

  if result != CL_SUCCESS {
    Err(Error::OpenCL(result, "clGetDeviceIDs"))?
  }

  if count == 0 {
    return Ok(vec![]);
  }

  let mut devices: Vec<cl_device_id> = vec![null_mut(); count as usize];

  let result: cl_int = clGetDeviceIDs(
    platform,
    CL_DEVICE_TYPE_GPU,
    count,
    devices.as_mut_ptr() as *mut cl_device_id,
    null_mut(),
  );

  if result != CL_SUCCESS {
    Err(Error::OpenCL(result, "clGetDeviceIDs"))?
  }

  Ok(devices)
}

#[inline]
pub unsafe fn get_device_info(device: cl_device_id, info: cl_device_info) -> Result<Vec<u8>, Error> {
  let mut size: size_t = 0;

  let result: cl_int = clGetDeviceInfo(device, info, 0, null_mut(), &mut size);

  if result != CL_SUCCESS {
    Err(Error::OpenCL(result, "clGetDeviceInfo"))?
  }

  if size == 0 {
    return Ok(vec![]);
  }

  let mut buffer: Vec<u8> = vec![0; size];

  let result: cl_int = clGetDeviceInfo(device, info, size, buffer.as_mut_ptr() as *mut c_void, null_mut());

  if result != CL_SUCCESS {
    Err(Error::OpenCL(result, "clGetDeviceInfo"))?
  }

  Ok(buffer)
}

#[inline]
pub unsafe fn get_program_build_info(program: cl_program, device: cl_device_id) -> Result<Vec<u8>, Error> {
  let mut size: size_t = 0;

  let result = clGetProgramBuildInfo(program, device, CL_PROGRAM_BUILD_LOG, 0, null_mut(), &mut size);

  if result != CL_SUCCESS {
    Err(Error::OpenCL(result, "clGetProgramBuildInfo"))?
  }

  if size == 0 {
    return Ok(vec![]);
  }

  let mut buffer: Vec<u8> = vec![0; size as usize];

  let result = clGetProgramBuildInfo(
    program,
    device,
    CL_PROGRAM_BUILD_LOG,
    size,
    buffer.as_mut_ptr() as *mut c_void,
    null_mut(),
  );

  if result != CL_SUCCESS {
    Err(Error::OpenCL(result, "clGetProgramBuildInfo"))?
  }

  Ok(buffer)
}

#[inline]
pub unsafe fn set_kernel_arg(
  kernel: cl_kernel,
  index: cl_uint,
  size: size_t,
  value: *const c_void,
) -> Result<(), Error> {
  let result: cl_int = clSetKernelArg(kernel, index, size, value);

  if result != CL_SUCCESS {
    Err(Error::OpenCL(result, "clSetKernelArg"))?
  }

  Ok(())
}

#[inline]
pub unsafe fn enqueue_write_buffer(
  queue: cl_command_queue,
  buffer: cl_mem,
  blocking: cl_bool,
  size: size_t,
  ptr: *const c_void,
) -> Result<(), Error> {
  let result: cl_int = clEnqueueWriteBuffer(queue, buffer, blocking, 0, size, ptr, 0, null_mut(), null_mut());

  if result != CL_SUCCESS {
    Err(Error::OpenCL(result, "clEnqueueWriteBuffer"))?
  }

  Ok(())
}

#[inline]
pub unsafe fn release_kernel(kernel: cl_kernel) -> Result<(), Error> {
  assert!(!kernel.is_null(), "Null pointer passed.");

  let result = clReleaseKernel(kernel);

  if result != CL_SUCCESS {
    Err(Error::OpenCL(result, "clReleaseKernel"))?
  }

  Ok(())
}

#[inline]
pub unsafe fn release_mem_object(memobj: cl_mem) -> Result<(), Error> {
  assert!(!memobj.is_null(), "Null pointer passed.");

  let result = clReleaseMemObject(memobj);

  if result != CL_SUCCESS {
    Err(Error::OpenCL(result, "clReleaseMemObject"))?
  }

  Ok(())
}

#[inline]
pub unsafe fn release_program(program: cl_program) -> Result<(), Error> {
  assert!(!program.is_null(), "Null pointer passed.");

  let result = clReleaseProgram(program);

  if result != CL_SUCCESS {
    Err(Error::OpenCL(result, "clReleaseProgram"))?
  }

  Ok(())
}

#[inline]
pub unsafe fn release_command_queue(queue: cl_command_queue) -> Result<(), Error> {
  assert!(!queue.is_null(), "Null pointer passed.");

  let result = clReleaseCommandQueue(queue);

  if result != CL_SUCCESS {
    Err(Error::OpenCL(result, "clReleaseCommandQueue"))?
  }

  Ok(())
}

#[inline]
pub unsafe fn release_context(context: cl_context) -> Result<(), Error> {
  assert!(!context.is_null(), "Null pointer passed.");

  let result = clReleaseContext(context);

  if result != CL_SUCCESS {
    Err(Error::OpenCL(result, "clReleaseContext"))?
  }

  Ok(())
}
