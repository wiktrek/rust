use std::process::Command;
fn main() {
    let output = "none";

    let mut list_dir = Command::new(r"cd target");
    list_dir.status().expect("process failed to execute");
    println!("{:?}", output);
}
