use crate::ffi::cl_uint;

#[derive(Debug)]
pub struct MinerConfig {
  pub devices: Vec<cl_uint>,
  pub memsizes: Vec<cl_uint>,
}

impl MinerConfig {
  #[inline]
  pub fn allowed_device(&self, index: usize, gindex: cl_uint) -> bool {
    if self.devices.is_empty() {
      return true;
    }

    self.devices.get(index).map(|&device| device == gindex).unwrap_or(false)
  }
}
