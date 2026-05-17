pub mod cd;
pub mod echo;
pub mod executable;
pub mod pwd;
pub mod r#type;

use std::process;

pub const BUILTIN_COMMANDS: [&str; 5] = ["type", "exit", "echo", "pwd", "cd"];

pub enum Command {
    Type(String),
    Exit,
    Echo(Vec<String>),
    Executable(Vec<String>),
    Pwd,
    Cd(String),
}

impl Command {
    pub fn from_input(input: &str) -> Command {
        crate::parser::parse(input)
    }

    pub fn execute(&self) {
        match self {
            Command::Type(command) => r#type::execute(command),
            Command::Echo(command) => echo::execute(command),
            Command::Executable(command) => executable::execute(command),
            Command::Pwd => pwd::execute(),
            Command::Cd(command) => cd::execute(command),
            Command::Exit => process::exit(0),
        }
    }
}
