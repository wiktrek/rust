use std::io;
use rand::Rng;
fn main() -> io::Result<()> {
    let num: String = nums_gen();
    guess(num.clone());
    println!("{}", num);

 
  
    Ok(())
}
fn guess(num: String) {


    let is_true = guessed_num(read_line(), num.clone());
    if is_true == true {
        println!("wow you guessed the number \n the number was: {}", num);
    } else {
        guess(num)
    }
}
fn generate_min_max() -> (i32, i32 ) {
    let mut rng = rand::thread_rng();
    let min: i32 = rng.gen_range(0..50);
    let max: i32 = rng.gen_range(min + 10..100);
return (min, max)
}
fn read_line() -> String{
    let mut user_input = String::new();
    println!("Enter a number");
    io::stdin()
    .read_line(&mut user_input).expect("Couldn’t read from stdin");

    return user_input.clone().replace("\r\n", "")
}
fn guessed_num(num: String, num2: String) -> bool{
    if num == num2 {
        return true
    }
// println!("{}, {}", num, num2);
    if num > num2 {
        println!("less")
    } else {
        println!("more")
    }
    return false
}
fn nums_gen() -> String{
    let mut rng = rand::thread_rng();
    let (min, max) = generate_min_max();
    let num: i32 = rng.gen_range(min..max);
println!("min: {}, max: {}", min , max);
    return num.to_string()
}
