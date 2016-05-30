extern crate mio;


use std::collections::VecDeque;

use mio::{EventLoop, EventSet, Token};
use mio::tcp::TcpStream;
use onion::{Onion, OnionCommand};



#[derive(Debug)]
pub struct Connection{
    socket: TcpStream,
    socket_status: mio::EventSet,
    token: mio::Token,
    read_buf: Vec<u8>,
    write_buf: Vec<u8>,
    read_eof: bool,
    failed: bool
}

impl Connection {
    pub fn new(socket: TcpStream, token: mio::Token) -> Connection {
        Connection{
            socket: socket,
            socket_status: mio::EventSet::none(),
            token: token,
            read_buf: Vec::with_capacity(1024),
            write_buf: Vec::with_capacity(1024),
            read_eof: false,
            failed: false
        }
    }

    pub fn handle_event(&mut self, ev_loop: &mut mio::EventLoop<Onion>, events: mio::EventSet) {
        self.socket_status.insert(events);
        info!("Connection handle event: {:?}, this time {:?}, now {:?}",
            self.socket.peer_addr(), events, self.socket_status)
    }

    pub fn register(&self, ev_loop: &mut mio::EventLoop<Onion>, token: mio::Token) {
        ev_loop.register(
            &self.socket,
            token,
            mio::EventSet::readable(),
            mio::PollOpt::edge() | mio::PollOpt::oneshot())
        .expect("event loop register");
    }

    pub fn is_closed(&self) -> bool {
        self.failed || (self.read_eof && self.write_buf.is_empty())
    }

    pub fn process_rules(&mut self, ev_loop: &mut mio::EventLoop<Onion>, parent_command: &mut VecDeque<OnionCommand>) {

    }
}
