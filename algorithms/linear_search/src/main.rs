fn main() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let mut a = 0;
    let searching_for = 25;
    while a < vec.len() {
        if vec[a] == searching_for {
            return println!("found at vec[{a}]");
        }
        a += 1;
    }
    println!("not found");
}