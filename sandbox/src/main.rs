mod print;
mod vars;
use print::run as Print;

fn main() {

println!("  main.rs:");
println!("Hello World");
Print("works");
vars::run();
}
