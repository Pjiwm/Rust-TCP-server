use super::string_algorithms;
use super::file_manager;
use colored::*;
use substring::Substring;

pub fn handler(cmd: &str) -> String {
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
        "reverse" => reverse(arg_len, args),
        "palindrome" => palindrome(arg_len, args),
        "scream" => scream(arg_len, args),
        "notes" => notes(arg_len, args, cmd),
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

fn reverse(arg_len: usize, args: Vec<&str>) -> String {
    if arg_len > 1 {
        return string_algorithms::reverse_string(args[1]);
    }
    return format!("{}", "Command error: No word was specified.\n".red());
}

fn palindrome(arg_len: usize, args: Vec<&str>) -> String {
    if arg_len > 1 {
        return string_algorithms::palindrome(args[1]);
    }
    return format!("{}", "Command error: No word was specified.\n".red());
}

fn scream(arg_len: usize, args: Vec<&str>) -> String {
    if arg_len > 1 {
        return string_algorithms::scream(args[1]);
    }
    return format!("{}", "Command error: No word was specified.\n".red());
}

fn notes(arg_len: usize, args: Vec<&str>, cmd: &str) -> String {
    {
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
}
