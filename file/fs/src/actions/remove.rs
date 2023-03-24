use std::{ io, fs};
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