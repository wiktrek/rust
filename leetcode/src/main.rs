
fn main() {
    let digits: Vec<i32> = vec![1, 2, 3];

    println!("{:?}", digits.last());
    let mut anwser: Vec<i32> = vec![];
    let mut split: i32 = format!("{:?}", digits).replace("[", "").replace("]", "").replace(", ", "").parse().unwrap();
    split += 1;
    let char_vec: Vec<char> = split.to_string().chars().collect();

    println!("{:?}", anwser);
    return;
}
