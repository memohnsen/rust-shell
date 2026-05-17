use crate::commands::Command;

pub fn parse(input: &str) -> Command {
    let input_arr: Vec<String> = input
        .split_ascii_whitespace()
        .map(|a| a.to_string())
        .collect();
    let mut args: Vec<String> = Vec::new();

    for arg in input_arr {
        args.push(arg);
    }

    match args[0].as_str() {
        "echo" => Command::Echo(args),
        "type" => Command::Type(args[1].clone()),
        "exit" => Command::Exit,
        "pwd" => Command::Pwd,
        "cd" => Command::Cd(args[1].clone()),
        _ => Command::Executable(args),
    }
}
