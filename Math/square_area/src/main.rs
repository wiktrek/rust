use std::io;
fn main() {
    let mut side = String::new();
    println!("Enter the side in cm");
    io::stdin()
    .read_line(&mut side).expect("Couldn’t read from stdin");
let a: f64 = side.replace("\r\n", "").parse().unwrap();
let area: f64 = a*a;
println!("{}cm²", area);
}
