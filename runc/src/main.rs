/*
    run gcc c/main.c -o main
    run main.exe
*/

use std::process::Command;
fn main() {
    println!("Hello, world!");
    let output = Command::new("gcc")
        .current_dir("../c")
        .arg("c/main.c -o main")
        .spawn()
        .expect("Failed to execute command");
}
