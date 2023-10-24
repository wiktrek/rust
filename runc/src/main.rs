

use std::process::Command;
fn main() {
    println!("Hello, world!");
    let filename = "main";
    let _gcc = Command::new("cmd")
        .args(["/C", &format!("gcc c/{}.c -o {}", filename, filename)])
        .spawn()
        .expect("failed to execute process");

    println!("Running {}.exe", filename);

    let _run = Command::new("cmd")
        .args(["/C", &format!("{filename}.exe")])
        .spawn()
        .expect("failed to execute process");
}
