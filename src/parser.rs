use crate::commands::Command;

pub fn parse(input: &str) -> Command {
    if input.starts_with("echo ") {
        let stripped = input.strip_prefix("echo ").unwrap();
        Command::Echo(String::from(stripped))
    } else if input.starts_with("type ") {
        let stripped = input.strip_prefix("type ").unwrap();
        Command::Type(String::from(stripped))
    } else if input == "exit" {
        Command::Exit
    } else if input == "pwd" {
        Command::Pwd
    } else if input.starts_with("cd ") {
        let stripped = input.strip_prefix("cd ").unwrap();
        Command::Cd(String::from(stripped))
    } else if input.starts_with("cat ") {
        let args: Vec<String> = input
            .split("'")
            .filter(|word| word != &"")
            .map(|s| s.to_string())
            .collect();
        Command::Executable(args)
    } else {
        let args: Vec<String> = input
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        Command::Executable(args)
    }
}
