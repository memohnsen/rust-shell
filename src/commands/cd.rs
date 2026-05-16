use std::{env, path::Path};

pub fn execute(command: &str) {
    let new_dir = Path::new(command);
    let home_dir = env::home_dir().expect("cd: {new_dir}: No such file or directory");

    if new_dir == "~" {
        match env::set_current_dir(home_dir) {
            Ok(_) => (),
            Err(_) => println!("cd: {}: No such file or directory", new_dir.display()),
        };
    } else {
        match env::set_current_dir(new_dir) {
            Ok(_) => (),
            Err(_) => println!("cd: {}: No such file or directory", new_dir.display()),
        };
    }
}
