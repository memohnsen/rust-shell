pub mod cat;
pub mod cd;
pub mod echo;
pub mod executable;
pub mod pwd;
pub mod r#type;

use std::process;

pub const BUILTIN_COMMANDS: [&str; 6] = ["type", "exit", "echo", "pwd", "cd", "cat"];

pub enum Command {
    Type(String),
    Exit,
    Echo(String),
    Executable(Vec<String>),
    Pwd,
    Cd(String),
    Cat(Vec<String>),
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
            Command::Cat(command) => cat::execute(command),
            Command::Pwd => pwd::execute(),
            Command::Cd(command) => cd::execute(command),
            Command::Exit => process::exit(0),
        }
    }
}
