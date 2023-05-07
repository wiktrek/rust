fn main() {
    // leetcode 746. Min Cost Climbing Stairs
    let cost = [1, 100, 1, 100, 400, 1];
    let len = cost.len();

    let mut n1 = cost[0];
    let mut n2 = cost[1];
    // loops from 2 to len ( 5)
    for stair in 2..len {
        let n3 = cost[stair] + n1.min(n2);
        // updating new states
        n1 = n2;
        n2 = n3
    }
    // result = min cost of reaching n-1 step and n-2
    println!("{}", n1.min(n2))
}
