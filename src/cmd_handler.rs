use super::file_manager;
use super::string_algorithms;
use colored::*;
use substring::Substring;

pub fn handler(cmd: &str, client_addr: &std::net::SocketAddr) -> String {
    let cmd_lower = cmd.to_lowercase().replace("\n", "");
    println!(
        "{} used: [{}]",
        format!("{}", client_addr).green(),
        cmd_lower.green()
    );
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
        "notes" => notes(arg_len, args, cmd, "notes"),
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

fn notes(arg_len: usize, args: Vec<&str>, cmd: &str, file_name: &str) -> String {
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
            return file_manager::read_file(file_name);
        }
        let start_idx = "notes write ".chars().count();
        let end_idx = cmd.chars().count() - 1;
        let new_line = cmd.substring(start_idx, end_idx);
        file_manager::write_file(file_name, new_line);
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

#[cfg(test)]
mod tests {
    use crate::cmd_handler;
    use colored::*;
    #[test]
    fn reverse_gives_error_when_lack_of_args() {
        let args = vec!["reverse"];
        let result = cmd_handler::reverse(args.len(), args);
        let expected_result = format!("{}", "Command error: No word was specified.\n".red());
        assert_eq!(result, expected_result);
    }

    #[test]
    fn reverse_works_when_enough_args_given() {
        let args = vec!["reverse", "jeff"];
        let result = cmd_handler::reverse(args.len(), args);
        assert_eq!(result, String::from("ffej\n"));
    }

    #[test]
    fn reverse_works_when_excessive_args_given() {
        let args = vec!["reverse", "jeff", "extra", "args"];
        let result = cmd_handler::reverse(args.len(), args);
        assert_eq!(result, String::from("ffej\n"))
    }

    #[test]
    fn palindrome_gives_error_when_lack_of_args() {
        let args = vec!["palindrome"];
        let result = cmd_handler::palindrome(args.len(), args);
        let expected_result = format!("{}", "Command error: No word was specified.\n".red());
        assert_eq!(result, expected_result);
    }

    #[test]
    fn palindrome_works_when_enough_args_given() {
        let args_true = vec!["palindrome", "legovogel"];
        let args_false = vec!["palindrome", "banana"];
        let expected_true = cmd_handler::palindrome(args_true.len(), args_true);
        let expected_false = cmd_handler::palindrome(args_false.len(), args_false);
        assert_eq!(expected_true, String::from("true\n"));
        assert_eq!(expected_false, String::from("false\n"));
    }

    #[test]
    fn palindrome_works_when_excessive_args_given() {
        let args_true = vec!["palindrome", "bob", "extra", "args"];
        let args_false = vec!["palindrome", "banana", "extra", "args"];
        let expected_true = cmd_handler::palindrome(args_true.len(), args_true);
        let expected_false = cmd_handler::palindrome(args_false.len(), args_false);
        assert_eq!(expected_true, String::from("true\n"));
        assert_eq!(expected_false, String::from("false\n"));
    }

    #[test]
    fn scream_gives_error_when_lack_of_args() {
        let args = vec!["scream"];
        let result = cmd_handler::scream(args.len(), args);
        let expected_result = format!("{}", "Command error: No word was specified.\n".red());
        assert_eq!(result, expected_result);
    }

    #[test]
    fn scream_works_when_enough_args_given() {
        let args = vec!["scream", "hello"];
        let result = cmd_handler::scream(args.len(), args);
        assert_eq!(result, String::from("HEEELLOOO!!!\n"));
    }

    #[test]
    fn scream_works_when_excessive_args_given() {
        let args = vec!["scream", "hello", "extra", "args"];
        let result = cmd_handler::scream(args.len(), args);
        assert_eq!(result, String::from("HEEELLOOO!!!\n"));
    }
}
