fn main() {
    let mut count = 0;
    let grid = vec![
        vec![1, 1, 1, 1, 0],
        vec![1, 1, 0, 1, 0],
        vec![1, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 1],
    ];
    let mut i = 0;
    while i < grid.len() {
        let mut j = 0;
        while j < grid[i].len() {
            if grid[i][j] == 1 {
                name(&grid);
                count += 1;
            }
            j += 1;
        }

        i += 1;
    }
}
fn name(grid: &Vec<Vec<i32>>, i: usize, j: usize) -> String {}
