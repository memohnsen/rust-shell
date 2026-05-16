pub fn execute(command: &str) {
    if command.starts_with("'") {
        println!("{}", command.trim_matches('\''));
    } else {
        let trimmed: String = command.split_whitespace().collect();
        println!("{trimmed}");
    }
}
