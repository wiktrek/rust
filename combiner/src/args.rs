fn get_nth_arg(n: unsize) -> String {
    std::env::args().nth(n).unwrap()

}
 pub struct Args {
    pub image_1: String,
    pub image_2: String,
    pub output: String,
}
impl Args {
    pub fn new() -> Self {
        Args {
            image_1: String::new(),
            image_2: String::new(),
            output: String::new(),
        } 

    }

}