use std::io;
use rand::Rng;
fn main() -> io::Result<()> {
    println!("Hello, world!");
    read_line();
Ok(())
}
fn read_line() -> String{
    let mut user_input = String::new();
    println!("Enter an input");
    io::stdin()
    .read_line(&mut user_input).expect("Couldn’t read from stdin");

    return user_input.clone().replace("\r\n", "")
}