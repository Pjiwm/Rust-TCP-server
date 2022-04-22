use super::cmd_handler;
use colored::*;
use std::io;
use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::{str, thread};

pub fn run_client(port: &str, ip: &str) {
    let mut stream = TcpStream::connect(format!("{}:{}", ip, port)).unwrap_or_else(|_| {
        panic!("Could not connect to server.");
    });

    let mut input_stream = stream.try_clone().unwrap();

    thread::spawn(move || loop {
        let mut client_buffer = [0u8; 1024];
        match input_stream.read(&mut client_buffer) {
            Ok(n) => {
                if n == 0 {
                    println!(
                        "{}\n{} {} {}",
                        "Disconnected".red(),
                        "Press:".yellow(),
                        "ctrl + c".green(),
                        "to exit.".yellow()
                    );
                    break;
                } else {
                    io::stdout().write(&client_buffer).unwrap();
                    io::stdout().flush().unwrap();
                }
            }
            Err(error) => panic!("{}", error.to_string()),
        }
    });

    let output_stream = &mut stream;

    loop {
        let mut user_buffer = String::new();

        io::stdin().read_line(&mut user_buffer).unwrap();

        user_buffer = user_buffer.replace("\r\n", "").replace("\n", "");

        output_stream
            .write(user_buffer.as_bytes())
            .unwrap_or_else(|_| 0);
        output_stream.flush().unwrap();
    }
}

pub fn tcp_listener(port: &str) {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port));
    // Opens a data stream...
    match listener {
        Ok(l) => {
            println!("{} {}", "Listening to port".yellow(), port.green());
            for stream in l.incoming() {
                match stream {
                    Ok(stream) => {
                        thread::spawn(move || {
                            let address = stream.peer_addr().unwrap();
                            println!(
                                "{} {}",
                                format!("{}", address).green(),
                                "Connected to client".yellow()
                            );
                            // connects to client
                            handle_client(stream, &address);
                        });
                    }
                    Err(_) => {
                        panic!("Error");
                    }
                }
            }
        }
        Err(err) => panic!("{}", err),
    }
}

fn handle_client(mut stream: TcpStream, client_adr: &std::net::SocketAddr) {
    loop {
        stream.write(b"> ").unwrap();
        let mut read = [0; 1028];
        match stream.read(&mut read) {
            Ok(bytes) => {
                if bytes == 0 {
                    // closed connection
                    break;
                }
                // command of the user as string
                let cmd = convert_bytes_to_str(&read[0..bytes]);
    
                // close connection with client when they use exit
                if cmd == "exit" || cmd.starts_with("exit ") {
                    break;
                }
                stream
                    .write(cmd_handler::handler(&cmd, client_adr).as_bytes())
                    .ok();
            }
            Err(err) => {
                panic!("{}", err);
            }
        }
    }
}

fn convert_bytes_to_str(buf: &[u8]) -> String {
    let s = match str::from_utf8(buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    s.to_owned()
}

#[cfg(test)]
mod tests {
    use crate::tcp;
    #[test]
    fn bytes_convert_correctly_to_string() {
        let string = String::from("This is a string");
        let bytes = &string.as_bytes();
        let converted_bytes = tcp::convert_bytes_to_str(bytes);
        assert_eq!(converted_bytes, string);
    }
}
