extern crate onion;

#[macro_use] extern crate log;
#[macro_use] extern crate clap;
use clap::{App, Arg};

#[cfg(test)]
mod tests {

    use onion;
    use clap::{Arg, App};

    use std;
    use std::io::prelude::*;

    use onion::{OnionMessageReader, OnionMessageWriter};

    const _SERVER_ : &'static str = "0.0.0.0:9090";

    fn start_server() {
        onion::init_log();
        std::thread::spawn( move ||
            onion::main(_SERVER_.parse().unwrap())
        );
        debug!("Server started");
    }



    #[test]
    fn connect() {
        start_server();

        std::thread::sleep_ms(5000);

        info!("Test");
        for i in 0..10 {
            let mut s = std::net::TcpStream::connect(_SERVER_).unwrap();
            match  OnionMessageWriter::new() {
                Ok(ret) => {
                    let _ = s.write(ret.as_slice());
                },
                Err(err) => assert!(false)
            }
            std::thread::sleep_ms(1000);
        }
        // let matches = App::new("stream").arg(
        //     Arg::with_name("bind")
        //     .short("l")
        //     .takes_value(true)
        //     .required(true)
        // ).get_matches();
        //
        // let address = value_t_or_exit!(matches.value_of("bind"), std::net::SocketAddr);


    }
}
