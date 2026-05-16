use find_in_path::FindInPath;
use std::{
    path::PathBuf,
    process::{self, Command, ExitStatus},
};

pub enum Commands {
    Type(String),
    Exit,
    Echo(String),
    Unknown(String),
}

impl Commands {
    pub fn from_input(input: &str) -> Commands {
        if input.starts_with("echo ") {
            let stripped = input.strip_prefix("echo ").unwrap();
            let args = String::from(stripped);
            Commands::Echo(args)
        } else if input.starts_with("type ") {
            let stripped = input.strip_prefix("type ").unwrap();
            let args = String::from(stripped);
            Commands::Type(args)
        } else if input == "exit" {
            Commands::Exit
        } else {
            Commands::Unknown(input.to_string())
        }
    }

    pub fn execute(&self) {
        const VALID_COMMANDS: [&str; 3] = ["type", "exit", "echo"];

        match &self {
            Commands::Type(command) => Self::handle_type_command(command, VALID_COMMANDS),
            Commands::Echo(command) => Self::handle_echo_command(command),
            Commands::Exit => process::exit(0),
            Commands::Unknown(command) => println!("{command}: not found"),
        }
    }

    pub fn handle_echo_command(command: &str) {
        // echo repeats the arg the user input
        println!("{command}");
    }

    pub fn handle_type_command(command: &str, valid_commands: [&str; 3]) {
        // find file or exe in path
        let command_in_path: Option<PathBuf> = command.to_string().find_in_path();

        // type says what the arg is, built in or not
        if valid_commands.contains(&command) {
            println!("{command} is a shell builtin");
        } else if let Some(path) = command_in_path {
            // go through every dir in path, check if file with command name exists
            // if exists and has execute perms, print ,else continue searching
            println!(
                "{command} is {}",
                path.to_str().expect("Error converitng file path to string")
            );
        } else {
            println!("{command}: not found");
        }
    }

    pub fn handle_executable_command(command: &str) {
        // find file or exe in path
        let command_in_path: Option<PathBuf> = command.to_string().find_in_path();

        // go through every dir in path, check if file with command name exists
        // if exists and has execute perms, execute
        if let Some(path) = command_in_path {
            let output = Command::new(path).arg("").arg("").output().expect("");
            let output_str = String::from_utf8_lossy(&output.stdout);
            println!("{output_str}");
        } else {
            println!("{command}: not found");
        }
    }
}
