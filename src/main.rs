extern crate lib_2048;

use std::net::TcpListener;
use std::env;
use lib_2048::data;

const default_port: usize = 4343;

fn main() {
    let port_requested: usize = match env::args().skip(1).next() {
        Some(p) => match p.parse() {
            Ok(port) => port,
            Err(_)   => default_port,
        },
        None    => default_port,
    };
    println!("{}", port_requested);
}
