use base64::encode;
use futures::sync::mpsc::unbounded;
use futures::sync::mpsc::UnboundedReceiver;
use futures::sync::mpsc::UnboundedSender;
use futures::Async;
use futures::Future;
use futures::Poll;
use futures::Sink;
use futures::Stream;
use nimiq_keys::Address;
use serde_json::from_str;
use serde_json::to_string;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread::spawn;
use tokio::runtime::current_thread;
use tokio_tungstenite::connect_async;
use tungstenite::Message;
use url::Url;

use crate::error::Error;
use crate::pool::PoolConfig;
use crate::pool::PoolMessage;
use crate::pool::PoolMode;
use crate::pool::PoolState;

type ArcState = Arc<RwLock<PoolState<PoolMessage>>>;
type Receiver = UnboundedReceiver<PoolMessage>;
type Sender = UnboundedSender<PoolMessage>;

#[derive(Debug)]
pub struct PoolClient {
  state: ArcState,
  sender: Sender,
}

impl PoolClient {
  pub fn new(config: &PoolConfig) -> Result<Self, Error> {
    let state = ArcState::default();

    let (sender, receiver) = unbounded();

    // use hash::Blake2bHash;
    // use hash::Hash;

    // let info = get_network_info(NetworkId::Main).unwrap();
    // let hash: Blake2bHash = info.genesis_block.header.hash();

    // TODO: FIXME
    let hash = [
      38, 74, 175, 138, 79, 152, 40, 167, 108, 85, 6, 53, 218, 7, 142, 180, 102, 48, 106, 24, 159, 204, 3, 113, 11,
      238, 159, 100, 156, 134, 157, 18,
    ];

    // TODO: Device Data
    //   startDifficulty
    //   deviceName
    //   minerVersion

    let address = Address::from_user_friendly_address(config.address)
      .unwrap()
      .to_user_friendly_address();

    // TODO: FIXME
    let device = 12345678;

    let message = PoolMessage::Register {
      address,
      device,
      data: None,
      mode: PoolMode::Nano,
      genesis: encode(&hash),
    };

    let register = Message::Text(to_string(&message)?);

    let read = Self::init_receiver(receiver, Arc::clone(&state));
    let send = Self::init_sender(sender.clone(), Arc::clone(&state), register, config.wsurl());

    spawn(move || current_thread::block_on_all(read));
    spawn(move || current_thread::block_on_all(send));

    Ok(Self { sender, state })
  }

  fn init_receiver(receiver: Receiver, state: ArcState) -> impl Future<Item = (), Error = ()> {
    receiver.for_each(move |message: PoolMessage| {
      state.write().unwrap().push(message);
      Ok(())
    })
  }

  fn init_sender(
    sender: Sender,
    state: ArcState,
    message: Message,
    url: Url,
  ) -> impl Future<Item = ((), ()), Error = Error> {
    let dispatch_message = move |message: Result<PoolMessage, Error>| {
      if let Ok(message) = message {
        sender.unbounded_send(message)?;
      } else {
        eprintln!("[x] WebSocket Message Error: {:?}", message);
      }

      Ok(())
    };

    let parse_message = |message: Message| -> Result<PoolMessage, Error> {
      match message {
        Message::Text(text) => {
          // println!("[+] JSON Text: {:?}", text);

          from_str(&text).map_err(Into::into)
        }
        _ => unreachable!(),
      }
    };

    connect_async(url)
      .map_err(Error::Connect)
      .map(|(duplex, _)| duplex.split())
      .and_then(move |(sink, stream)| {
        println!("[+] WebSocket Handshake Successfully Completed");

        state.write().unwrap().activate();

        stream
          .filter(|event| event.is_text())
          .map_err(Error::Read)
          .map(parse_message)
          .for_each(dispatch_message)
          .join(sink.send(message).map_err(Error::Send).map(|_| ()))
      })
  }
}

impl Stream for PoolClient {
  type Item = PoolMessage;
  type Error = Error;

  fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
    let mut state = self.state.write().unwrap();

    if state.is_active() {
      if let Some(message) = state.pop() {
        return Ok(Async::Ready(Some(message)));
      }
    }

    state.set_task();

    Ok(Async::NotReady)
  }
}
