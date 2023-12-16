pub fn reverse_vec(mut vec: Vec<&str>) -> Vec<&str> {
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
pub fn check_reversed(string: String) -> bool {
    // if string == string.chars().rev().collect::<String>() {
    //     return true;
    // }
    let vec = string.chars().collect::<Vec<char>>();
    let mut i = vec.len() - 1;
    let mut n = 0;
    while n <= vec.len() {
        if vec[n] != vec[i] {
            return false;
        }
        if i == n {
            return true;
        }
        i -= 1;
        n += 1;
    }
    false
}
