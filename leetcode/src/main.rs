mod remove_element;
mod serach_insert;
use remove_element::remove_element;
use serach_insert::search_insert;
fn main() {
    let nums = &mut vec![1,2,3,4,5];
remove_element(nums, 4);
search_insert(vec![1,4,5], 3);
}
