pub fn execute(command: &str) {
    if command.starts_with("'") {
        let trimmed: String = command.split("'").collect();
        println!("{}", trimmed);
    } else {
        let trimmed: String = command.split_whitespace().collect::<Vec<&str>>().join(" ");
        println!("{trimmed}");
    }
}
