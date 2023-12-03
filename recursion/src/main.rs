fn main() {
    println!("Hello, world!");
    print_vec(vec!["recursion", "is", "easy", "!"], 0)
}
fn print_vec(vec: Vec<&str>, i: usize) {
    if vec.len() <= i {
        return;
    }
    println!("{}", vec[i]);
    print_vec(vec, i + 1);
}
