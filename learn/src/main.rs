fn main() {
    println!("{:?}", reverse_vec(vec!["1", "2", "3", "4", "5"]))
}
fn reverse_vec(mut vec: Vec<&str>) -> Vec<&str> {
    let mut n = 0;
    let mut l = vec.len() - 1;
    while l <= vec.len() {
        if n == l {
            return vec;
        }
        vec.swap(n, l);
        n += 1;
        l -= 1;
        if n == (l + 1) {
            return vec;
        }
    }
    vec
}
