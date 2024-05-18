
fn main() {
    println!("Enter number");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    print!("{line}");
    let num: i32 = line.replace("\r\n", "").parse::<i32>().unwrap();
    let mut primes: Vec<i32> = Vec::new();
    let start = std::time::Instant::now();
    for i in 2..num {
        if check_prime(primes.clone(), i) {
            primes.push(i);
        }
    }
    eprintln!("{:?}", start.elapsed());
    println!("{}", primes.len());
}
fn check_prime(primes: Vec<i32>, num: i32) -> bool {
    for prime in primes {
            if num % prime == 0 {
                return false;
            }
    }
true
}