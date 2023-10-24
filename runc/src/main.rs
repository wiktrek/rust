/*
    run gcc c/main.c -o main
    run main.exe
*/

use std::process::Command;
fn main() {
    println!("Hello, world!");
    let filename = "main";
    let _output = Command::new("gcc")
        .current_dir("./")
        .arg(format!("c/{}.c -o {}", filename, filename))
        .spawn()
        .expect("Failed to execute command");
    println!("Run {}.exe", filename)
}
