use std::{ io, fs};
fn main() {
    let mut action: String = String::new(); 
    
    println!("Hello, world!");
io::stdin()
    .read_line(&mut action).expect("Couldn’t read from stdin");
    let a: &str = action.parse().unwrap();
    match a {
    "ez" => rename()
    }
}
fn rename() {

}
