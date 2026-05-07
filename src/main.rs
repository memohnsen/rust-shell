use find_in_path::prelude::*;
use std::io::{self, Write};
use std::path::PathBuf;

fn handle_echo_command(stripped_command: &str) {
    // echo repeats the arg the user input
    println!("{stripped_command}");
}

fn handle_type_command(stripped_command: &str, valid_commands: [&str; 3]) {
    let command_in_path: Option<PathBuf> = stripped_command.to_string().find_in_path();

    // type says what the arg is, built in or not
    if valid_commands.contains(&stripped_command) {
        println!("{stripped_command} is a shell builtin");
    } else if let Some(path) = command_in_path {
        // go through every dir in path, check if file with command name exists
        // if exists and has execute perms, print ,else continue searching
        println!("{stripped_command} is { }", path.to_str().expect("Invalid"));
    } else {
        println!("{stripped_command}: not found");
    }
}

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
            handle_echo_command(stripped);
        } else if let Some(stripped_command) = command.strip_prefix("type ") {
            handle_type_command(stripped_command, VALID_COMMANDS);
        } else {
            println!("{command}: command not found");
        };
    }
}
