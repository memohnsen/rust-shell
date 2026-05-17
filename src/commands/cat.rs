use find_in_path::FindInPath;
use std::{os::unix::process::CommandExt, path::PathBuf, process::Command};

pub fn execute(command: &[String]) {
    if command.is_empty() {
        return;
    }

    let command_in_path: Option<PathBuf> = command[0].find_in_path();
    let args = &command[1..];

    if let Some(path) = command_in_path {
        let output = Command::new(path)
            .arg0(&command[0])
            .args(args)
            .output()
            .expect("Error running program");
        let output_str = String::from_utf8_lossy(&output.stdout);
        print!("{output_str}");
    } else {
        println!("{}: not found", command[0].trim());
    }
}
