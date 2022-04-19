use colored::*;
use std::env;
mod cmd_handler;
mod file_manager;
mod string_algorithms;
mod tcp;

fn main() {
    let client_msg = format!(
        "To use as a client use args:\n{}",
        "{client} {port} {ip}".green()
    );
    let server_msg = format!(
        "To use as a server use args:\n{}",
        "{server} {port}".green()
    );
    println!("Starting up...\n{}\n{}\n\n", client_msg, server_msg);

    let args: Vec<String> = env::args().collect();

    let app_type = check_app_type(&args);
    let port = check_port(&args);

    // Run app type
    if app_type.to_lowercase() == "server" {
        tcp::tcp_listener(port)
    } else if app_type.to_lowercase() == "client" {
        let ip = check_ip(&args);
        tcp::run_client(port, ip);
    } else {
        panic!(
            "Unknown app type was specified, choose between: {} or {}",
            String::from("client").green(),
            String::from("server").green()
        );
    }
}

fn check_app_type(args: &Vec<String>) -> &str {
    let app_type = args
        .get(1)
        .ok_or_else(|| {
            panic!(
                "App type wasn't specified, choose between: {} or {}",
                String::from("client").green(),
                String::from("server").green()
            );
        })
        .unwrap();
    app_type
}

fn check_port(args: &Vec<String>) -> &str {
    let port = args
        .get(2)
        .ok_or_else(|| {
            panic!("port wasn't specified");
        })
        .unwrap();

    for c in port.chars() {
        if !c.is_numeric() {
            panic!("port contained non numeric characters")
        }
    }
    let port_int = port.parse::<i32>().unwrap();
    if port_int < 1024 || port_int > 65535 {
        panic!(
            "port number should be between {} and {}",
            "1024".yellow(),
            "65535".yellow()
        )
    }
    port
}

fn check_ip(args: &Vec<String>) -> &str {
    let ip = args.get(3).unwrap_or_else(|| {
        panic!("ip wasn't specified");
    });
    ip
}
