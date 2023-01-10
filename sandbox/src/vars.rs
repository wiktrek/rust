
pub fn run() {
    
    let mut name: &str= "Brad";
    let mut age: i32 = 30;
    println!("{} is {} years old", name, age);
    age = 50;
    name = "John";
    println!("{} is {} years old", name, age);
    const ID: i32 = 001;
    println!("ID: {}", ID);
    let ( my_name, my_age) = ("Brad",37);
    println!("{} is {} years old", my_name, my_age);
}