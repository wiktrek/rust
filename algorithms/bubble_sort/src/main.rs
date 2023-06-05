fn main() {
    let mut a = vec![34, 42, 42, 12, 521, 1, 4, 52, 21, 41234, 521];
    let mut i = 0;
    let n = a.len();
    while i < n - 1 {
        let mut j = n - 1;
        while j > 0 {
            if a[j] < a[j - 1] {
                a.swap(j, j - 1)
            }
            j -= 1;
        }
        i += 1;
    }
    println!("{:?}", a);
}
