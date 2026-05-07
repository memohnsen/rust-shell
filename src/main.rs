use find_in_path::prelude::*;
use std::io::{self, Write};
use std::path::PathBuf;

fn main() {
    const VALID_COMMANDS: [&str; 3] = ["type", "exit", "echo"];

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("{command}: command not found");

        let command = command.trim();

        if command == "exit" {
            break;
        } else if let Some(stripped) = command.strip_prefix("echo ") {
            // echo repeats the arg the user input
            println!("{stripped}");
        } else if let Some(stripped_command) = command.strip_prefix("type ") {
            // go through every dir in path, check if file with command name exists
            // if exists and has execute perms, print ,else continue searching
            let stripped_command = stripped_command.to_string();
            let command_in_path: Option<PathBuf> = stripped_command.find_in_path();

            if let Some(path) = command_in_path {
                println!("command is {:?}", path)
            } else {
                println!("{stripped_command}: not found");
            }
        } else {
            println!("{command}: command not found");
        };
    }
}
