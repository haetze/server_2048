extern crate lib_2048;

use std::net::TcpListener;
use std::net::TcpStream;
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
    match TcpListener::bind("127.0.0.1:8080") {
        Err(_) => println!("Port unavailable, restart with different port."),
        Ok(listener) => {
            match listener.accept() {
                Ok((socket, _)) => handle_messages(socket),
                Err(_)          => println!("Fail to accept connection, try again."),
            }
        },
    }
    
}

fn handle_messages(socket: TcpStream) {
    
    
}
