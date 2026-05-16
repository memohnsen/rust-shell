pub fn execute(command: &str) {
    if command.starts_with("'") {
        println!("{}", command.trim_matches('\''));
    } else {
        println!("{command}");
    }
}
