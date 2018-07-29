extern crate lib_2048;

mod commands;

use std::net::TcpListener;
use std::net::TcpStream;
use std::env;
use std::io::Read;
use std::io::Write;
use lib_2048::data::Field;
use commands::Command;



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

fn handle_messages(mut socket: TcpStream) {
    let mut field = None;

    loop {
        let mut command = String::new();
        socket.read_to_string(&mut command);
        match command.trim() {
            "right" => handle_command(&mut field, Command::Right, &mut socket),
            "left"  => handle_command(&mut field, Command::Left , &mut socket),
            "up"    => handle_command(&mut field, Command::Up   , &mut socket),
            "down"  => handle_command(&mut field, Command::Down , &mut socket),
            other   => {
                let commands: Vec<&str> = other.split_whitespace().collect();
                if let "new" = commands[0] {
                    let scale = match commands[1].parse::<usize>() {
                        Ok(n) => n,
                        Err(_) => 4,
                    };
                    
                    handle_command(&mut field, Command::New(scale), &mut socket)
                } else {
                    socket.write(b"Unsupported Command");
                }
            },
        }
    }
}


fn handle_command(mut field_option: &mut Option<Field>, command: Command, mut socket: &mut TcpStream) {
    use std::mem::swap;
    
    let mut field = None;
    swap(&mut field, &mut field_option);

    let mut result_field = match field {
        None => {
            match command {
                Command::New(n) => Some(Field::new(n)),
                _               => None,
            }
        },
        Some(mut field) => {
            match command {
                Command::New(n) => Some(Field::new(n)),
                Command::Right  => {
                    field.swipe_right();
                    Some(field)
                },
                Command::Left  => {
                    field.swipe_left();
                    Some(field)
                },
                Command::Up  => {
                    field.swipe_up();
                    Some(field)
                },
                Command::Down  => {
                    field.swipe_down();
                    Some(field)
                },
            }
        },
    };
    print_result(&result_field, &mut socket);
    swap(&mut result_field, &mut field_option);

}

fn print_result(field: &Option<Field>, socket: &mut TcpStream) {
    
}
 
