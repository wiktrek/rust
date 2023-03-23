use std::{ io, fs};
fn main() {
    let mut action: String = String::new(); 
    
    println!("Hello, world!");
io::stdin()
    .read_line(&mut action).expect("Couldn’t read from stdin");
    let replace = action.replace("\r\n", "").replace(" ", "");
    let a: &str = replace.as_str();
    match a {
    "create" => create(),
    "rename" => rename(),
    _ => println!("error, picked: {}", a)
    }
}
fn rename() {
println!("rename")
}
fn create() {
    println!("create")
}
fn remove() {
    println!("remove")
}