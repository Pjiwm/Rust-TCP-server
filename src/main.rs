use colored::*;
use std::env;
mod string_algorithms;
mod cmd_handler;
mod file_manager;
mod tcp;

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

    tcp::tcp_listener(argument);
}
