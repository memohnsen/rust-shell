use find_in_path::FindInPath;
use std::path::PathBuf;

use super::BUILTIN_COMMANDS;

pub fn execute(command: &str) {
    let command_in_path: Option<PathBuf> = command.to_string().find_in_path();

    if BUILTIN_COMMANDS.contains(&command) {
        println!("{command} is a shell builtin");
    } else if let Some(path) = command_in_path {
        println!(
            "{command} is {}",
            path.to_str().expect("Error converting file path to string")
        );
    } else {
        println!("{command}: not found");
    }
}
