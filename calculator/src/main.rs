use std::env::{args, Args};
fn main() {
let args: Args= args();
let first: Option<String> = args.nth(0);
println!("{:?}", first)
}
