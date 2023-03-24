use std::{ io, fs};
fn main() -> Result<(), std::io::Error> {
    let mut action: String = String::new(); 
    
    println!("Hello, world!");
io::stdin()
    .read_line(&mut action).expect("Couldn’t read from stdin");
    let replace = action.replace("\r\n", "").replace(" ", "");
    let a: &str = replace.as_str();
    match a {
    "create" => create(),
    "remove" => remove(),
    "rename" => return rename(),
    _ => println!("error, picked: {}", a)
    }
}
fn rename() -> Result<(), std::io::Error> {
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

    fs::rename(replace_path, replace_path_to);


println!("rename");
    Ok(())
}
fn create() ->  Result<(), std::io::Error>{
    println!("create");
    Ok(())
}
fn remove() ->  Result<(), std::io::Error> {
    println!("remove")
    Ok(())
}