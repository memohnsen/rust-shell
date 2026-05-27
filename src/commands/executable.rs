use find_in_path::FindInPath;
use std::{os::unix::process::CommandExt, path::PathBuf, process::Command};

pub fn execute(command: &[String]) {
    if command.is_empty() {
        return;
    }

    let command_in_path: Option<PathBuf> = command[0].find_in_path();

    if let Some(path) = command_in_path {
        // Redirection is handled by our shell, not by the external command.
        // For: ls -1 /tmp/dog > /tmp/cow/ant.md
        // command is: ["ls", "-1", "/tmp/dog", ">", "/tmp/cow/ant.md"]
        // TODO: find the position of ">" or "1>" in command.

        // Only pass real command arguments to the program. The redirect
        // operator and filename belong to the shell, so they are not args.
        // TODO: if redirect exists, args should stop before the redirect.
        // TODO: if redirect does not exist, args should be all tokens after the command name.
        let args = &command[1..];

        let output = Command::new(path)
            .arg0(&command[0])
            .args(args)
            .output()
            .expect("Error running program");

        // The filename comes right after ">" or "1>". stdout goes into
        // this file instead of being printed to the terminal.
        // TODO: when redirect exists, write output.stdout to that filename.

        // No redirect means stdout should appear in the terminal.
        // TODO: only print stdout here when there is no redirect.
        print!("{}", String::from_utf8_lossy(&output.stdout));

        // ">" and "1>" only redirect stdout. stderr still appears in the terminal.
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
    } else {
        println!("{}: not found", command[0]);
    }
}
