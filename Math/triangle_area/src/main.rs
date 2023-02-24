use std::io;
fn main() {
    let mut base = String::new();
    println!("Enter the base in cm");
    io::stdin()
    .read_line(&mut base).expect("Couldn’t read from stdin");
let mut height = String::new();
println!("Enter the height in cm");
io::stdin()
.read_line(&mut height).expect("Couldn’t read from stdin");
let a: f64 = base.replace("\r\n", "").parse().unwrap();
let h: f64 = height.replace("\r\n", "").parse().unwrap();
let area: f64 = ( a * h ) / 2.0;
println!("{}cm²", area);
}
