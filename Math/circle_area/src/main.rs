use std::io;
fn main() {
    let mut user_input = String::new();
    println!("Enter the radius of a circle in cm");
    io::stdin()
    .read_line(&mut user_input).expect("Couldn’t read from stdin");
let pi: f64 = 3.14159265359;
let a: f64 = user_input.replace("\r\n", "").parse().unwrap();
println!("{}", (pi*(a*a)));
}
