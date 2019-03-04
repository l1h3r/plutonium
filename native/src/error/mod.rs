use crate::ffi::cl_int;

#[derive(Debug)]
pub enum Error {
  Bytes(usize, usize),
  OpenCL(cl_int, &'static str),
  Custom(String),
}

impl From<&str> for Error {
  fn from(other: &str) -> Self {
    other.to_owned().into()
  }
}

impl From<String> for Error {
  fn from(other: String) -> Self {
    Error::Custom(other)
  }
}
