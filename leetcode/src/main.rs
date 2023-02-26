fn main() {
    let a = "101";
    let b = "1";
    let n: String = "egnikasd".to_string();
    // println!("{}", format!("{:b}",a.parse::<i32>().unwrap()))
    // println!("{:?}", i32::from_str_radix(a, 2).unwrap())
    let c =  i32::from_str_radix(b, 2).unwrap() + i32::from_str_radix(a, 2).unwrap();
    println!("{}", format!("{:b}", c));

}
