use std::ops::Deref;
use std::os::raw::c_char;
use std::os::raw::c_uchar;

#[derive(Debug)]
#[repr(C)]
pub struct Source(*const c_char);

//
// Warning: BYO Safety
//
unsafe impl Send for Source {}
unsafe impl Sync for Source {}

impl Source {
  #[inline]
  pub fn new(source: &'static [c_uchar]) -> Self {
    Self(source.as_ptr() as *const c_char)
  }
}

impl Deref for Source {
  type Target = *const c_char;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
