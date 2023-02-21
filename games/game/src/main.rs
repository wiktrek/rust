use std::io;

fn main() -> io::Result<()> {
    let mut user_input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut user_input).expect("Couldn’t read from stdin");

    println!("input {} ", user_input);
    Ok(())
}
