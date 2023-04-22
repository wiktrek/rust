use std::{fs, io};
pub fn create() -> Result<(), std::io::Error> {
    println!("create");

    let mut path: String = String::new();

    println!("path to the directory you want to create");

    io::stdin()
        .read_line(&mut path)
        .expect("Couldn’t read from stdin");
    println!("path to the directory you want to rename to");

    let replace_path = path.replace("\r\n", "").replace(" ", "");

    let result = fs::create_dir(replace_path);

    result
}
