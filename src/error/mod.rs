use base64::DecodeError as Decode64;
use beserial::SerializingError as BESerial;
use futures::sync::mpsc::SendError;
use nimiq_lib::error::ClientError;
use serde::Deserialize;
use serde::Deserializer;
use serde_json::Error as Serde;
use std::io::Error as IOError;
use tungstenite::error::Error as Tungstenite;

use crate::ffi::cl_int;
use crate::pool::PoolMessage;

#[derive(Debug)]
pub enum Error {
  Bytes(usize, usize),
  OpenCL(cl_int, &'static str),
  Connect(Tungstenite),
  Send(Tungstenite),
  Read(Tungstenite),
  Custom(String),
  Serde(Serde),
  SendMessage(SendError<PoolMessage>),
  Decode64(Decode64),
  BESerial(BESerial),
  NimiqClient(ClientError),
  IO(IOError),
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

impl From<Serde> for Error {
  fn from(other: Serde) -> Self {
    Error::Serde(other)
  }
}

impl From<SendError<PoolMessage>> for Error {
  fn from(other: SendError<PoolMessage>) -> Self {
    Error::SendMessage(other)
  }
}

impl From<Decode64> for Error {
  fn from(other: Decode64) -> Self {
    Error::Decode64(other)
  }
}

impl From<BESerial> for Error {
  fn from(other: BESerial) -> Self {
    Error::BESerial(other)
  }
}

impl From<ClientError> for Error {
  fn from(other: ClientError) -> Self {
    Error::NimiqClient(other)
  }
}

impl From<IOError> for Error {
  fn from(other: IOError) -> Self {
    Error::IO(other)
  }
}

impl<'de> Deserialize<'de> for Error {
  fn deserialize<D: Deserializer<'de>>(_deserializer: D) -> Result<Self, D::Error> {
    unimplemented!();
  }
}
