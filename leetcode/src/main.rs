fn main() {
    // leetcode 49
    use std::collections::HashMap;
    let strs = ["eat".to_string(), "tea".to_string()];
    let mut h = HashMap::new();
    for s in strs {
        let mut key: Vec<char> = s.chars().collect();
        key.sort();
        h.entry(key).or_insert(vec![]).push(s);
        println!("{:?}", h)
    }
    let result: Vec<Vec<String>> = h.values().cloned().collect();
    println!("{:?}", result)
}
