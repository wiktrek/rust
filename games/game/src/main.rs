use std::io;

fn main() -> io::Result<()> {
    
    let nums: Vec<String> = nums_gen();

check_nums(read_line(), nums.clone());
    println!("floor {:?}", nums);
  
    Ok(())
}
fn check_nums(num: String, nums: Vec<String>) -> bool {




return false
}
fn read_line() -> String{
    let mut user_input = String::new();
    println!("Enter a number");
    io::stdin()
    .read_line(&mut user_input).expect("Couldn’t read from stdin");

    return user_input.clone().replace("\r\n", "")
}
fn nums_gen() -> Vec<String>{
    // floor: "0"..."9"
    let mut floor: Vec<String> = vec![];
    let mut a = 0;
    while a < 10 {
        floor.push(a.to_string());
        a += 1;
    }
    return floor
}
