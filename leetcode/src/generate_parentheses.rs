pub fn generate_parenthesis(n: i32) -> Vec<String> {
    fn gen_recursive(cache: &mut Vec<String>, curr: String, remain_left: i32, remain_right: i32) {
        if remain_left == 0 && remain_right == 0 {
            cache.push(curr);
            return;
        }
        if remain_left > 0 {
            gen_recursive(cache, curr.clone() + "(", remain_left - 1, remain_right);
        }
        if remain_left < remain_right {
            gen_recursive(cache, curr.clone() + ")", remain_left, remain_right - 1);
        }
    }

    let mut ans = vec![];
    gen_recursive(&mut ans, String::new(), n, n);
    println!("{:?}", ans);
    ans
}