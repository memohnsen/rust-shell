pub fn execute(command: &str) {
    let mut trimmed: String = String::new();

    if command.starts_with("'") {
        trimmed = command.split("'").collect();
    } else {
        trimmed = command.split_whitespace().collect::<Vec<&str>>().join(" ");
    }

    println!("{trimmed}");
}
