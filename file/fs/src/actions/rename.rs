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
