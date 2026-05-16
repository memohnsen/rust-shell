use codecrafters_shell::Commands;
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

        let commmand_value = Commands::from_input(command);

        commmand_value.execute();
    }
}
