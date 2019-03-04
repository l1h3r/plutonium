#[macro_use]
extern crate neon;

pub mod config;
pub mod error;
pub mod ffi;
pub mod hash;
pub mod miner;
pub mod opencl;
pub mod utils;

use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
  Ok(cx.string("hello node"))
}

register_module!(mut cx, {
  cx.export_function("hello", hello)
});
