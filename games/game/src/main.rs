use std::io;

fn main() -> io::Result<()> {
    let mut floor: Vec<String> = floor_gen();

    
    floor.push(read_line());
    println!("floor {:?}", floor);
  
    Ok(())
}
fn read_line() -> String{
    let mut user_input = String::new();
    println!("Enter a number");
    io::stdin()
    .read_line(&mut user_input).expect("Couldn’t read from stdin");

    return user_input.clone().replace("\r\n", "")
}
fn floor_gen() -> Vec<String>{
    // floor: "0"..."9"
    let mut floor: Vec<String> = vec![];
    let mut a = 0;
    while a < 11 {
        floor.push(a.to_string());
        a += 1;
    }
    return floor
}
