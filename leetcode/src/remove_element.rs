pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let expected: Vec<&i32> = Vec::new();
    nums.sort();
    for n in nums {
        if n != &val {
expected.push(n);
        }

    }
        return expected.len() as i32;
}