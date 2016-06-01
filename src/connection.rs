extern crate mio;


use std::collections::VecDeque;

use mio::{EventLoop, EventSet, Token};
use mio::tcp::TcpStream;
use mio::{TryRead,TryWrite};
use onion::{Onion, OnionCommand};
use rcp::OnionMessageReader;



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
        info!("Process rules connection");
        if self.socket_status.is_readable() {
            self.read();
            self.socket_status.remove(mio::EventSet::readable());
        }
    }

    fn read(&mut self) {
        let mut buffer = [0u8;512];
        match self.socket.try_read(&mut buffer) {
            Ok(Some(0)) => {
                info!("{:?}: EOF!", self.socket.peer_addr() );
                self.read_eof = true;
            },
            Ok(Some(n)) => {
                info!("{:?}: Read {}bytes", self.socket.peer_addr(), n);
                match OnionMessageReader::new_from_buffer(&buffer) {
                    Ok(msg) => match msg.get() {
                        Ok(s) => info!("{:?}", s.get_command()),
                        Err(e) => error!("{:?}", e)
                    },
                    Err(e) => error!("Error {:?}", e)
                };
            },
            Ok(None) => {
                info!("{:?}: Noop!", self.socket.peer_addr());
            },
            Err(e) => {
                error!("got an error trying to read; err={:?}", e);
                self.failed =true;
            }

        };
    }
}
