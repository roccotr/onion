extern crate mio;


use std::collections::VecDeque;

use mio::*;
use mio::tcp::{TcpListener, TcpStream};
use onion::{Onion, OnionCommand};


#[derive(Debug)]
pub struct Listener{
    listener: TcpListener,
    sock_status: mio::EventSet
}

impl Listener {
    pub fn new(listener: TcpListener) -> Listener {
        Listener {
            listener: listener,
            sock_status: mio::EventSet::none()
        }
    }

    pub fn handle_event(&mut self, ev_loop: &mut mio::EventLoop<Onion>, events: mio::EventSet) {
        assert!(events.is_readable());
        self.sock_status.insert(events);
        info!("Listener::handle_event: {:?}; this time: {:?}; now: {:?}",
               self.listener.local_addr(), events, self.sock_status);
    }

    pub fn register(&self, ev_loop: &mut mio::EventLoop<Onion>, token: mio::Token) {
        ev_loop.register(
            &self.listener,
            token,
            mio::EventSet::readable(),
            mio::PollOpt::edge()
        ).expect("Register listener");
    }

    pub fn is_closed(&self) -> bool {
        false
    }

    pub fn process_rules(&mut self, ev_loop: &mut mio::EventLoop<Onion>, parent_command: &mut VecDeque<OnionCommand>) {
        if self.sock_status.is_readable() {
            info!("the listener socket is ready to accept a connection");
            match self.listener.accept() {
                Ok(Some((socket, addr))) => {
                    parent_command.push_back(OnionCommand::NewConnection(socket, addr));
                    info!("add new connection");
                },
                Ok(None) => {
                    info!("the listener socket wasn't actually ready");
                },
                Err(e) => {
                    info!("listener.accept() errored: {}", e);
                    ev_loop.shutdown();
                }
            };
            self.sock_status.remove(mio::EventSet::readable());
        }
    }
}
