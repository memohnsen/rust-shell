use std::io::{self, Write};

fn main() {
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
        } else {
            println!("{command}: command not found");
        };
    }
}
