use std::{ io, fs};
fn main() -> Result<(), std::io::Error>{
    println!("Hello, world!");
start()
}
fn start() -> Result<(), std::io::Error>{
    let mut action: String = String::new(); 

io::stdin()
    .read_line(&mut action).expect("Couldn’t read from stdin");
    let replace = action.replace("\r\n", "").replace(" ", "");
    let a: &str = replace.as_str();
    match a {
    "create" => create(),
    "remove" => remove(),
    "rename" => return rename(),
    "options" => options(),
    _ => panic!("error, picked: {}", a)
    }
}
fn rename() -> Result<(), std::io::Error> {
    let mut path: String = String::new(); 
    let mut path_to: String = String::new(); 

    println!("path to the directory you want to rename");

io::stdin()
    .read_line(&mut path).expect("Couldn’t read from stdin");
    println!("path to the directory you want to rename to"); 
io::stdin()
    .read_line(&mut path_to).expect("Couldn’t read from stdin");


    let replace_path = path.replace("\r\n", "").replace(" ", "");
    let replace_path_to = path_to.replace("\r\n", "").replace(" ", "");

    let result = fs::rename(replace_path, replace_path_to);


println!("rename");
    result
}
fn options() -> Result<(), std::io::Error> {
    println!("rename");
    print!("create");
    println!("remove");

    Ok(())
}
fn create() ->  Result<(), std::io::Error>{
    println!("create");

    let mut path: String = String::new(); 

    println!("path to the directory you want to create");

io::stdin()
    .read_line(&mut path).expect("Couldn’t read from stdin");
    println!("path to the directory you want to rename to"); 


    let replace_path = path.replace("\r\n", "").replace(" ", "");

    let result = fs::create_dir(replace_path);


    result
}
fn remove() ->  Result<(), std::io::Error> {
    println!("remove");

    let mut path: String = String::new(); 

    println!("path to the directory you want to create");

io::stdin()
    .read_line(&mut path).expect("Couldn’t read from stdin");
    println!("path to the directory you want to rename to"); 


    let replace_path = path.replace("\r\n", "").replace(" ", "");

    let result = fs::remove_dir(replace_path);


    result
}