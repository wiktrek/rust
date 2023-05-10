fn main() {
    // 200.Number of Islands
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
                name(&grid, i, j);
                count += 1;
            }
            j += 1;
        }

        i += 1;
    }
}
fn name(grid: &Vec<Vec<i32>>, i: usize, j: usize) {
    let mut grid_copy = grid.clone();
    if i >= grid.len() || j >= grid[i].len() || grid[i][j] == 0 {
        return;
    }
    grid_copy[i][j] = 0;
    name(&grid_copy, i + 1, j);
    name(&grid_copy, i - 1, j);
    name(&grid_copy, i, j + 1);
    name(&grid_copy, i, j - 1);
}
