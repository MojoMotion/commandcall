use std::process::Command;
use std::str;

fn main() {
    let output = Command::new("echo")
        .arg("Hello world")
        .output()
        .expect("Failed to execute command");

    let result = match str::from_utf8(&output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("{}", result);
}
