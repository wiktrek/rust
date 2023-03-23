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
    "remove" => remove(),
    "rename" => rename(),
    _ => println!("error, picked: {}", a)
    }
}
fn rename() {
    let mut path: String = String::new(); 
    let mut path_to: String = String::new(); 

    println!("path to the directory you want to rename");

io::stdin()
    .read_line(&mut path).expect("Couldn’t read from stdin");
io::stdin()
    .read_line(&mut path_to).expect("Couldn’t read from stdin");

    println!("path to the directory you want to rename to"); 
    let replace_path = path.replace("\r\n", "").replace(" ", "");
    let replace_path_to = path_to.replace("\r\n", "").replace(" ", "");




println!("rename");
    let rename = fs::rename("", "");
    println!("{:?}", rename);
    
}
fn create() {
    println!("create")
}
fn remove() {
    println!("remove")
}