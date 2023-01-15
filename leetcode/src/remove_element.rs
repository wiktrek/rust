pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.sort();
    for n in nums {
        if n != &val {
nums.remove(n as u32);
        }

    }
        return nums.len() as i32;
}