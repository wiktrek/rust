use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use rocket::*;

#[get("/save/<user>/<data>")]
pub fn save(user: String, data: String) -> String{
let path = Path::new("src/files/text.txt").with_file_name(format!("{}.txt", user.clone()));
println!("{:?}", path);
let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create: {}", why),
        Ok(file) => file,
    };

    match file.write_all(data.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    };
    format!("data:{data}, user: {user}")
}
#[get("/delete/<user>")]
pub fn delete(user: String) -> String{
    let path = Path::new("src/files/text.txt").with_file_name(format!("{}.txt", user.clone()));
        match fs::remove_file(&path) {
            Err(why) => return format!("{}", why),
            Ok(file) => file,
        };
    
        format!("deleted user: {user}")
    }