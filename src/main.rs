use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::{env, str, thread};

mod cmd;

fn main() {
    let args: Vec<String> = env::args().collect();
    // validate if a correct port was given
    let argument = args
        .get(1)
        .ok_or_else(|| {
            panic!("port wasn't specified");
        })
        .unwrap();

    for c in argument.chars() {
        if !c.is_numeric() {
            panic!("port contained non numeric characters")
        }
    }
    let port_int = argument.parse::<i32>().unwrap();
    if port_int < 1024 || port_int > 65535 {
        panic!("port number should be between 1024 and 65535")
    }

    tcp_listener(argument);
}

fn handle_client(mut stream: TcpStream) {
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
                stream.write(cmd_handler(&cmd).as_bytes()).ok();
            }
            Err(err) => {
                panic!("{}", err);
            }
        }
    }
}

fn tcp_listener(port: &str) {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port));
    // Opens a data stream...
    match listener {
        Ok(l) => {
            println!("Listening to port {}", port);
            for stream in l.incoming() {
                match stream {
                    Ok(stream) => {
                        thread::spawn(move || {
                            println!("Connected to client");
                            // connects to client
                            handle_client(stream);
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

fn convert_bytes_to_str(buf: &[u8]) -> String {
    let s = match str::from_utf8(buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    s.to_owned()
}

fn cmd_handler(cmd: &str) -> String {
    let cmd_lower = cmd.to_lowercase().replace("\n", "");
    let args: Vec<&str> = cmd_lower.split(" ").collect();
    println!("client used: [{}]", args[0]);

    let contains_arg1: bool = args.len() > 1;

    match args[0] {
        "help" => {
            return "available commands: \n reverse {word} \n palindrome {word} \n scream {word}\n"
                .to_owned()
        }
        "reverse" => {
            if contains_arg1 {
                return cmd::reverse_string(args[1]);
            }
            return "Command error: No word was specified.\n".to_owned();
        }
        "palindrome" => {
            if contains_arg1 {
                return cmd::palindrome(args[1]);
            }
            return "Command error: No word was specified.\n".to_owned();
        }
        "scream" => {
            if contains_arg1 {
                return cmd::scream(args[1]);
            }
            return "Command error: No word was specified.\n".to_owned();
        }
        _ => return format!("Unknown command: '{}'. Type help for help.\n", args[0]),
    }
}
