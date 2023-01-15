pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut total = nums.len();
    nums.sort();
    for n in (0..total).rev()  {
        if nums.is_empty() && nums[n] == val {
            nums.remove(n);
            total -= 1;
        }

    }
    println!("{total}");
        return total as i32;
}