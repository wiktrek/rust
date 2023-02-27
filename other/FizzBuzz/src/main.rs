fn main() {
    let mut a = 0;

    while a < 100 {
        let mut output = "".to_string();
        a += 1;
        if a % 3 == 0 { 
            output = format!("{}Fizz", output)
        }
        if a % 5 == 0 { 
            output = format!("{}Buzz", output)
        }
        if output == "" {output = a.to_string()}
        println!("{}", output)
    }

}
