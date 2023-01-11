pub fn sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    println!("  two_sum.rs:");
    let mut res: Vec<i32> = Vec::with_capacity(2);
    for  i in 0..nums.len() {

        for n in i+1..nums.len() {
            if nums[i]+nums[n]==target {
                res.push(i as i32);
                res.push(n as i32);
            }
        }
    }
    let one: usize = res[0].try_into().unwrap();
    let two: usize = res[1].try_into().unwrap();
println!("{:?} {:?} + {:?} = {target}, target: {target}", nums, nums[one], nums[two]);
    res
}