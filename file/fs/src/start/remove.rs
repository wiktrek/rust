use std::{ io, fs};

pub fn remove() ->  result<(), std::io::error> {
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