use super::cmd;
use super::file_manager;
use colored::*;
use substring::Substring;

pub fn reverse(arg_len: usize, args: Vec<&str>) -> String {
    if arg_len > 1 {
        cmd::reverse_string(args[1]);
    }
    return format!("{}", "Command error: No word was specified.\n".red());
}

pub fn palindrome(arg_len: usize, args: Vec<&str>) -> String {
    if arg_len > 1 {
        cmd::palindrome(args[1]);
    }
    return format!("{}", "Command error: No word was specified.\n".red());
}

pub fn scream(arg_len: usize, args: Vec<&str>) -> String {
    if arg_len > 1 {
        cmd::scream(args[1]);
    }
    return format!("{}", "Command error: No word was specified.\n".red());
}

pub fn notes(arg_len: usize, args: Vec<&str>, cmd: &str) -> String {
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
