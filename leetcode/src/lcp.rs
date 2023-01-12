pub fn longest_common_prefix(strs: Vec<&str>) -> String {
    println!("  lcp.rs:");
    let mut res = String::from(strs[0]);

    for str in strs {
        while !str.starts_with(&res) {
            res = res.chars().take(res.len() - 1).collect();
        }
    }
print!("{res}");
    return res;
}
