fn main() {
    let a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut lo = 0;
    let mut a_len = a.len();
    let searching = 4;

    while lo <= a_len {
        let m = (lo + (a_len - lo) / 2);
        let v = a[m];
        match v {
            _searching => return println!("Found {}", m),
            _ => a_len = m,
        }
    }

    println!("Not found")
}
