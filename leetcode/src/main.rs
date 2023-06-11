fn main() {
    // leetcode 242

    let s = "te";
    let t = "et";
    let mut a: Vec<u8> = s.bytes().collect();
    let mut b: Vec<u8> = t.bytes().collect();
    a.sort();
    b.sort();
    if a == b {
        print!("true")
    } else {
        print!("false")
    }
    // print!("{:?}, {:?}", a, b);
}
