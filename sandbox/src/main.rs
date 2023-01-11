mod print;
mod vars;
mod convert;
use print::run as Print;
use convert::convert;
fn main() {

println!("  main.rs:");
println!("Hello World");
Print("works");
vars::run();
convert("VI");

}
