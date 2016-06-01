extern crate mio;

use mio::tcp::{TcpListener, TcpStream};
use mio::util::Slab;

use std::collections::VecDeque;

use listener::Listener;
use connection::Connection;


#[derive(Debug)]
pub enum OnionCommand {
    NewConnection(TcpStream, ::std::net::SocketAddr),
    Broadcast(String)
}

#[derive(Debug)]
enum EventHandler{
    Listener (Listener),
    Conn (Connection)
}

impl EventHandler {

    fn handle_event(&mut self, ev_loop: &mut mio::EventLoop<Onion>, events: mio::EventSet) {
        match self {
            &mut EventHandler::Conn(ref mut conn) => conn.handle_event(ev_loop, events),
            &mut EventHandler::Listener(ref mut listener) => listener.handle_event(ev_loop, events)
        }
    }

    fn register(&self, ev_loop: &mut mio::EventLoop<Onion>, token: mio::Token) {
        match self {
            &EventHandler::Conn(ref conn) => conn.register(ev_loop, token),
            &EventHandler::Listener(ref listener) => listener.register(ev_loop, token)
        }
    }

    fn is_closed(&self) -> bool {
        match self {
            &EventHandler::Conn(ref conn) => conn.is_closed(),
            &EventHandler::Listener(ref listener) => listener.is_closed()
        }
    }

    fn process_rules(&mut self, ev_loop: &mut mio::EventLoop<Onion>, command_parent: &mut VecDeque<OnionCommand>) {
        match self {
            &mut EventHandler::Conn(ref mut conn) => conn.process_rules(ev_loop, command_parent),
            &mut EventHandler::Listener(ref mut listener) => listener.process_rules(ev_loop, command_parent)
        }
    }
}


pub struct Onion {
    connections: Slab<EventHandler>
}

impl Onion {
    pub fn new() -> Onion {
        Onion{
            connections: Slab::new(1024)
        }
    }

    pub fn listen(&mut self, ev_loop: &mut mio::EventLoop<Self>, listener: TcpListener) {
        let ev = EventHandler::Listener(Listener::new(listener));
        let token = self.connections.insert(ev).expect("Insert listener");
        self.connections[token].register(ev_loop, token);
    }

    pub fn process_action(&mut self, action: OnionCommand, ev_loop: &mut mio::EventLoop<Self>) {
        trace!("{:p}; got {:?}", self, action);

        match action {
            OnionCommand::NewConnection(socket, addr) => {
                let token = self.connections.insert_with( |token| {
                    EventHandler::Conn(Connection::new(socket, token))
                }).expect("token insert");
                self.connections[token].register(ev_loop, token);
            },
            OnionCommand::Broadcast(String) => {

            }
        }
    }


}

impl mio::Handler for Onion {
    type Timeout = ();
    type Message = ();
    fn ready(&mut self, ev_loop: &mut mio::EventLoop<Self>, token: mio::Token, events: mio::EventSet) {
        info!("Ready {:?} - {:?}", token, events);
        self.connections[token].handle_event(ev_loop, events);
        if self.connections[token].is_closed() {
            info!("Removing connection: {:?}", token);
            self.connections.remove(token);
        }

        let mut parent_command : VecDeque<OnionCommand> = VecDeque::new();
        self.connections[token].process_rules(ev_loop, &mut parent_command);

        for action in parent_command.drain(..) {
            self.process_action(action, ev_loop);
        }

    }
}
