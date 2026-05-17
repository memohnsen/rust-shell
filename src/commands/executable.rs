use find_in_path::FindInPath;
use std::{fs::File, io::Write, os::unix::process::CommandExt, path::PathBuf, process::Command};

pub fn execute(command: &[String]) {
    if command.is_empty() {
        return;
    }

    let command_in_path: Option<PathBuf> = command[0].find_in_path();

    if command.contains(&">".to_string()) || command.contains(&"1>".to_string()) {
        let mut file = File::create(&command[command.len() - 1]).unwrap();
        let output = command[1..command.len() - 2].join(" ");

        file.write_all(output.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    } else if let Some(path) = command_in_path {
        let args = &command[1..];

        let output = Command::new(path)
            .arg0(&command[0])
            .args(args)
            .output()
            .expect("Error running program");
        let output_str = String::from_utf8_lossy(&output.stdout);
        print!("{output_str}");
    } else {
        println!("{}: not found", command[0]);
    }
}
