mod reverse;
use reverse::*;
fn main() {
    println!("{:?}", reverse_vec(vec!["1", "2", "3", "4", "5"]));
    println!("{}", check_reversed("adstsda".to_string()));
}
