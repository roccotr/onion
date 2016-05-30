extern crate mio;
// extern crate log4rs;
extern crate onion;

// #[macro_use] extern crate log;
#[macro_use] extern crate clap;
//
//
// use mio::tcp::*;
// use mio::EventLoop;
// use mio::{TryRead, TryWrite};
// use mio::util::Slab;
use clap::{Arg, App};


fn main() {
    onion::init_log();

    let matches = App::new("stream").arg(
        Arg::with_name("bind")
        .short("l")
        .takes_value(true)
        .required(true)
    ).get_matches();

    let address = value_t_or_exit!(matches.value_of("bind"), std::net::SocketAddr);

    onion::main(address);
}
