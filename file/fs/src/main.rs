use crate::start::start;

mod start;
fn main() -> Result<(), std::io::Error>{
    println!("Hello, world!");
start::start()
}
