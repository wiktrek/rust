pub fn climb(n: i32) -> i32 {
let mut solutions: i32 = 0;
let mut one_step_before = 2;
let mut two_steps_before = 1;
if n <= 0 {
    return 0;
}
if n == 1 {
    return 1
}
if n == 2 { 
    return 2; 
}

for _ in 2..n {
    println!("{solutions}, {one_step_before}, {two_steps_before}");
solutions = one_step_before + two_steps_before;
two_steps_before = one_step_before;
one_step_before = solutions;

}
return solutions;
}