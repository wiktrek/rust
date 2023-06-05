use std::cmp::Ordering;
fn main() {
    // the array needs to be sorted
    // a is sorted
    let a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut lo = 0;
    let mut a_len = a.len();
    let searching = 4;

    while lo <= a_len {
        let m = lo + (a_len - lo) / 2;
        let v = a[m];
        match v.cmp(&searching) {
            Ordering::Greater => a_len = m,
            Ordering::Equal => return println!("Found {m} {v}"),
            Ordering::Less => lo = m + 1,
        }
        // if v == searching {
        //     return println!("Found {m} {v}");
        // } else if v > searching {
        //     a_len = m;
        // } else {
        //     lo = m + 1;
        // }
    }

    println!("Not found")
}
