use std::mem::uninitialized;
use std::mem::size_of;
use std::ptr::copy;

use crate::error::Error;

pub unsafe fn bytes_into<T>(vec: Vec<u8>) -> Result<T, Error> {
  if size_of::<T>() == vec.len() {
    let mut result: T = uninitialized();

    copy(
      vec.as_ptr(),
      &mut result as *mut T as *mut u8,
      vec.len(),
    );

    Ok(result)
  } else {
    Err(Error::Bytes(vec.len(), size_of::<T>()))
  }
}

pub fn bytes_into_string(mut bytes: Vec<u8>) -> String {
  if bytes.last() == Some(&0) {
    bytes.pop();
  }

  String::from_utf8(bytes)
    .map(|string| String::from(string.trim()))
    .unwrap()
}
