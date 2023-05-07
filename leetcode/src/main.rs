fn main() {
    let mut n1: i32 = 0;
    let mut n2: i32 = 0;
    let l1 = [1, 2, 3];
    let l2 = [4, 5, 6];
    for (index, i) in l1.iter().enumerate() {
        println!("{}, {}", i, index);
        n1 = n1 + i * i32::pow(10, index as u32);
    }
    println!("{}", n1);
    let output = [1];
}
