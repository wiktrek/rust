use std::env::{args, Args};
fn main() {
let mut args: Args= args();
let pops: String = args.nth(1).unwrap();
let time:String= args.nth(0).unwrap();
}