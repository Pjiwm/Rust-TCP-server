use colored::*;
use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::{env, str, thread};
use substring::Substring;

mod authenticate;
mod cmd;
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
        "reverse" => {
            if arg_len > 1 {
                return cmd::reverse_string(args[1]);
            }
            return format!("{}", "Command error: No word was specified.\n".red());
        }
        "palindrome" => {
            if arg_len > 1 {
                return cmd::palindrome(args[1]);
            }
            return format!("{}", "Command error: No word was specified.\n".red());
        }
        "scream" => {
            if arg_len > 1 {
                return cmd::scream(args[1]);
            }
            return format!("{}", "Command error: No word was specified.\n".red());
        }
        "notes" => {
            if !(arg_len > 1) {
                return format!(
                    "{}{}{}, {}\n",
                    "Command error: No argument was was specified.\n".red(),
                    "Available arguments: ".yellow(),
                    "write".green(),
                    "read".green()
                );
            }
            let arg2 = &args[1].to_ascii_lowercase();
            if arg2 != "write" && arg2 != "read" {
                return format!(
                    "{}{}{}, {}\n",
                    "Command error: Unknown argument was specified.\n".red(),
                    "Available arguments: ".yellow(),
                    "write".green(),
                    "read".green()
                );
            }

            if !(arg_len > 2) && arg2 != "read" {
                return format!(
                    "{}{} {} {}\n",
                    "Command error: writing requires extra argument.\n".red(),
                    "Usage:",
                    "notes write".yellow(),
                    "This is a new note".green()
                );
            }

            if arg2 == "read" {
                return file_manager::read_file("notes");
            }
            let start_idx = "notes write ".chars().count();
            let end_idx = cmd.chars().count() - 1;
            let new_line = cmd.substring(start_idx, end_idx);
            file_manager::write_file("notes", new_line);
            if new_line.chars().count() < 3 {
                return format!(
                    "{}{} {} {}\n",
                    "Command error: new note has to be at least 3 characters.\n".red(),
                    "Usage:",
                    "notes write".yellow(),
                    "This is a new note".green()
                );
            }
            return format!("{}", "Line added to notes.\n".green());
        }
        _ => {
            return format!(
                "{}{}{}\n",
                "Unknown command: '".yellow(),
                args[0].red(),
                "'. Type help for help.".yellow()
            )
        }
    }
}
