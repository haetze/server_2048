#![allow(unused_must_use)]

extern crate lib_2048;

mod commands;

use std::net::TcpListener;
use std::net::TcpStream;
use std::net::SocketAddr;
use std::env;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use lib_2048::data::Field;
use commands::Command;



const DEFAULT_PORT: u16 = 4343;



fn main() {
    let mut number_of_handled_connection = 0;
    let port_requested: u16 = match env::args().skip(1).next() {
        Some(p) => match p.parse() {
            Ok(port) => port,
            Err(_)   => DEFAULT_PORT,
        },
        None    => DEFAULT_PORT,
    };
    println!("Running on Port: {}", port_requested);
    match TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], port_requested))) {
        Err(_) => println!("Port unavailable, restart with different port."),
        Ok(listener) => loop {
            match listener.accept() {
                Ok((socket, _)) => {
                    number_of_handled_connection = number_of_handled_connection + 1;
                    std::thread::spawn(move || {
                    let socket = BufReader::new(socket);
                        handle_messages(socket);
                        println!("Close #{} connection", number_of_handled_connection);
                    });
                },
                Err(_)          => println!("Fail to accept connection, try again."),
            }
        },
    }
    
}

fn handle_messages(mut socket: BufReader<TcpStream>) {
    let mut field = None;

    loop {
        let mut command = String::new();
        match socket.read_line(&mut command) {
            Ok(0) => break,
            Ok(_) => {},
            Err(_) => {
                println!("Error while reading");
                break;
            },
        };

        match command.trim() {
            "right" => handle_command(&mut field, Command::Right, &mut socket),
            "left"  => handle_command(&mut field, Command::Left , &mut socket),
            "up"    => handle_command(&mut field, Command::Up   , &mut socket),
            "down"  => handle_command(&mut field, Command::Down , &mut socket),
            "exit"  => break,
            other   => {
                let commands: Vec<&str> = other.split_whitespace().collect();
                if commands.len() == 0 { continue; }
                if let "new" = commands[0] {
                    let scale = match commands[1].parse::<usize>() {
                        Ok(n) => n,
                        Err(_) => 4,
                    };
                    
                    handle_command(&mut field, Command::New(scale), &mut socket)
                } else {
                    socket.get_mut().write(b"Unsupported Command\n");
                }
            },
        }
    }
}


fn handle_command(mut field_option: &mut Option<Field>, command: Command, mut socket: &mut BufReader<TcpStream>) {
    use std::mem::swap;
    
    let mut field = None;
    swap(&mut field, &mut field_option);

    let tmp_field = field.clone();
    
    let result_field = match field {
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
    
    let mut result_field = result_field.map(|mut f| {
        match tmp_field {
            None => {
                f.insert_random();
            },
            Some(field) => {
                if field != f {
                    f.insert_random();
                }
            },       
        }
        f
    });
        
                
       
    
    print_result(&result_field, &mut socket);
    swap(&mut result_field, &mut field_option);

}

fn print_result(field: &Option<Field>, socket: &mut BufReader<TcpStream>) {
    match field {
        None => {
            socket.get_mut().write(b"Empty\n");
        },

        Some(field) => {
            let mut string = String::new();
            for row in &field.rows {
                let s = format!("{:?}", row.row);
                string.push_str(&s);
                string.push_str(&";");
            }
            string.push_str(&"\n");
            socket.get_mut().write(string.as_bytes());
        },
    }
}
 
