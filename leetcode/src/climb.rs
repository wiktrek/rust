pub fn climb(n: i32) -> i32 {
let mut solutions: i32 = 0;
let mut one_step_before = 2;
let mut two_steps_before = 1;
for _ in 0..n {
    println!("{solutions}, {one_step_before}, {two_steps_before}");
solutions = one_step_before + two_steps_before;
two_steps_before = one_step_before;
one_step_before = solutions;

}
return solutions;
}