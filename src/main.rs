use std::io::{self, Write};

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
            println!("{stripped}");
        } else if let Some(stripped_command) = command.strip_prefix("type ") {
            if VALID_COMMANDS.contains(&stripped_command) {
                println!("{stripped_command} is a shell builtin");
            } else {
                println!("{command}: command not found");
            }
        } else {
            println!("{command}: command not found");
        };
    }
}
