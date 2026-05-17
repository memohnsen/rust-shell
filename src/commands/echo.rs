use std::{fs::File, io::Write};

pub fn execute(command: &[String]) {
    if command.contains(&">".to_string()) || command.contains(&"1>".to_string()) {
        let mut file = File::create(&command[command.len() - 1]).unwrap();
        let output = command[1..command.len() - 2].join(" ");

        file.write_all(output.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    } else {
        println!("{}", command[1..].join(" "));
    }
}
