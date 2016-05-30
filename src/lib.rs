extern crate capnp;
extern crate capnpc;


mod onion_capnp {
    include!(concat!("../schema", "/onion_capnp.rs"));
}

//pub use onion_capnp::{Command, message};


#[macro_use] extern crate log;
extern crate log4rs;

extern crate mio;

use mio::tcp::{TcpListener};

const LOG_FILE : &'static str = "log.toml";

pub fn init_log() {
    if let Err(e) = log4rs::init_file(LOG_FILE, Default::default()) {
        panic!("Could not init logger from file: {} : {}", LOG_FILE, e);
    }
}

pub fn main (address: std::net::SocketAddr) {
    let listener = TcpListener::bind(&address).expect("bind");

    let mut ev_loop = mio::EventLoop::new().expect("Can't create event loop");

    let mut service  = Onion::new();

    service.listen(&mut ev_loop, listener);

    info!("running at: {:?}", address);

    ev_loop.run(&mut service).expect("run loop");
}
mod onion;
mod listener;
mod connection;
mod rcp;

pub use onion::Onion;
pub use rcp::{OnionMessageReader, OnionMessageWriter};
