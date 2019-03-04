use std::os::raw::c_char;
use std::os::raw::c_void;

use crate::ffi::types::*;

#[cfg_attr(target_os = "macos", link(name = "OpenCL", kind = "framework"))]
extern "system" {
  pub fn clGetPlatformIDs(
    num_entries: cl_uint,
    platforms: *mut cl_platform_id,
    num_platforms: *mut cl_uint
  ) -> cl_int;

  pub fn clGetPlatformInfo(
    platform: cl_platform_id,
    param_name: cl_platform_info,
    param_value_size: size_t,
    param_value: *mut c_void,
    param_value_size_ret: *mut size_t,
  ) -> cl_int;

  pub fn clGetDeviceIDs(
    platform: cl_platform_id,
    device_type: cl_device_type,
    num_entries: cl_uint,
    devices: *mut cl_device_id,
    num_devices: *mut cl_uint,
  ) -> cl_int;

  pub fn clGetDeviceInfo(
    device: cl_device_id,
    param_name: cl_device_info,
    param_value_size: size_t,
    param_value: *mut c_void,
    param_value_size_ret: *mut size_t,
  ) -> cl_int;

  pub fn clCreateContext(
    properties: *const cl_context_properties,
    num_devices: cl_uint,
    devices: *const cl_device_id,
    pfn_notify: Option<extern fn (*const c_char, *const c_void, size_t, *mut c_void)>,
    user_data: *mut c_void,
    errcode_ret: *mut cl_int,
  ) -> cl_context;

  pub fn clReleaseContext(context: cl_context) -> cl_int;

  pub fn clCreateBuffer(
    context: cl_context,
    flags: cl_mem_flags,
    size: size_t,
    host_ptr: *mut c_void,
    errcode_ret: *mut cl_int,
  ) -> cl_mem;

  pub fn clReleaseMemObject(memobj: cl_mem) -> cl_int;

  pub fn clCreateCommandQueue(
    context: cl_context,
    device: cl_device_id,
    properties: cl_command_queue_properties,
    errcode_ret: *mut cl_int
  ) -> cl_command_queue;

  pub fn clReleaseCommandQueue(command_queue: cl_command_queue) -> cl_int;

  pub fn clCreateProgramWithSource(
    context: cl_context,
    count: cl_uint,
    strings: *const *const c_char,
    lengths: *const size_t,
    errcode_ret: *mut cl_int,
  ) -> cl_program;

  pub fn clReleaseProgram(program: cl_program) -> cl_int;

  pub fn clBuildProgram(
    program: cl_program,
    num_devices: cl_uint,
    device_list: *const cl_device_id,
    options: *const c_char,
    pfn_notify: Option<extern fn (cl_program, *mut c_void)>,
    user_data: *mut c_void,
  ) -> cl_int;

  pub fn clGetProgramBuildInfo(
    program: cl_program,
    device: cl_device_id,
    param_name: cl_program_build_info,
    param_value_size: size_t,
    param_value: *mut c_void,
    param_value_size_ret: *mut size_t,
  ) -> cl_int;

  pub fn clCreateKernel(
    program: cl_program,
    kernel_name: *const c_char,
    errcode_ret: *mut cl_int,
  ) -> cl_kernel;

  pub fn clSetKernelArg(
    kernel: cl_kernel,
    arg_index: cl_uint,
    arg_size: size_t,
    arg_value: *const c_void,
  ) -> cl_int;

  pub fn clReleaseKernel(kernel: cl_kernel) -> cl_int;

  pub fn clEnqueueReadBuffer(
    command_queue: cl_command_queue,
    buffer: cl_mem,
    blocking_read: cl_bool,
    offset: size_t,
    cb: size_t,
    ptr: *mut c_void,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
    event: *mut cl_event,
  ) -> cl_int;

  pub fn clEnqueueWriteBuffer(
    command_queue: cl_command_queue,
    buffer: cl_mem,
    blocking_write: cl_bool,
    offset: size_t,
    cb: size_t,
    ptr: *const c_void,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
    event: *mut cl_event,
  ) -> cl_int;

  pub fn clEnqueueNDRangeKernel(
    command_queue: cl_command_queue,
    kernel: cl_kernel,
    work_dim: cl_uint,
    global_work_offset: *const size_t,
    global_work_dims: *const size_t,
    local_work_dims: *const size_t,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
    event: *mut cl_event,
  ) -> cl_int;
}
