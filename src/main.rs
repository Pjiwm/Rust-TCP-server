use colored::*;
use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::{env, str, thread};

mod cmd;
mod cmd_handler;
mod file_manager;

fn main() {
    file_manager::write_file("users", "I love icecream");
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
        panic!(
            "port number should be between {} and {}",
            "1024".yellow(),
            "65535".yellow()
        )
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

                // check if in authentication phase.
                // command of the user as string
                let cmd = convert_bytes_to_str(&read[0..bytes]);
                // close connection with client when they use exit
                if cmd == "exit\n" || cmd.starts_with("exit ") {
                    stream
                        .write(format!("{}", "Disconnected\n".yellow()).as_bytes())
                        .ok();
                    break;
                }
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
    println!("client used: [{}]", cmd_lower.green());
    let args: Vec<&str> = cmd_lower.split(" ").collect();

    let arg_len = args.len();

    match args[0] {
        "help" => {
            return format!(
                "{} \n reverse {} \n palindrome {} \n scream {}\n notes {} {}\nexit",
                "available commands:".yellow().underline(),
                "{word}".green(),
                "{word}".green(),
                "{word}".green(),
                "{read/write}".green(),
                "{Your new note}".green()
            )
        }
        "reverse" => cmd_handler::reverse(arg_len, args),
        "palindrome" => cmd_handler::palindrome(arg_len, args),
        "scream" => cmd_handler::scream(arg_len, args),
        "notes" => cmd_handler::notes(arg_len, args, cmd),
        _ => {
            format!(
                "{}{}{}\n",
                "Unknown command: '".yellow(),
                args[0].red(),
                "'. Type help for help.".yellow()
            )
        }
    }
}
