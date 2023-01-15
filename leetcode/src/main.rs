mod remove_element;
use remove_element::remove_element;
fn main() {
    let nums = &mut vec![1,2,3,4,5];
remove_element(nums, 4);
}
