use std::process::Command;
#[tokio::main]
async fn main() {
    let filename = "fib";

    let mut command = Command::new("cmd");
    if let Ok(mut child) = command
        .args(["/C", &format!("gcc c/{}.c -o {}", filename, filename)])
        .spawn()
    {
        child.wait().expect("command wasn't running");
        println!("Run {}.exe", filename)
        // println!("Running {}.exe", filename);
        // let _run = Command::new("cmd")
        //     .args(["/C", &format!("{filename}.exe")])
        //     .spawn()
        //     .expect("failed to execute process");
    } else {
        println!("Command failed");
    }
}
