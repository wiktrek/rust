
pub fn run() {
    println!("  vars.rs:");
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

    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
    println!("Max i128: {}", std::i128::MAX);

    let mut is_active: bool = true;
    if is_active == true {
        is_active = false;
    println!("is active? {}", is_active);

    }
}