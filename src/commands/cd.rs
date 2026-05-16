use std::{env, path::Path};

pub fn execute(command: &str) {
    // TODO: handle all path types:
    // absolute - full path from /
    // relative - ./ ../
    // ~ from home dir
    //
    // for absolute, if dir go to dir
    // else cd: dir: No such file or directory

    let new_dir = Path::new(command);
    match env::set_current_dir(new_dir) {
        Ok(_) => (),
        Err(_) => println!("cd: {}: No such file or directory", new_dir.display()),
    };
}
