/*
    0 -> nothing
    1 -> snake
    2 -> apple
    3 -> snake head
*/
use std::thread::sleep;
use std::time::{Duration};
#[derive(Debug, PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
    n: i32,
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    West, East, North, South
}

#[derive(Debug, PartialEq, Clone)]
struct Snake {
    head: Point,
    body: Vec<Point>,
    facing: Direction
}
fn main() {
    println!("Hello, world!");
    let mut snake: Snake;
    let mut grid = generate_grid(10);
    // println!("{:?}", grid);
    
    (grid, snake) = snake_head(grid);
    loop {
    (grid, snake) = animate(grid, snake);
sleep(Duration::new(1, 0))
    }

}
fn generate_grid(max: i32) -> Vec<Point> {
    let mut grid: Vec<Point> = vec![];
    println!("grid will have {} points", max.pow(2));
    for i in 0..max {
        for j in 0..max {
            grid.push(
                Point {
                    x: j,
                    y: i,
                    n: 0,
                }
            )
        }
    }

    grid
}
fn snake_head(mut grid: Vec<Point>) -> (Vec<Point>, Snake) {
    let snake = Snake {
        head: Point {
            x: 0,
            y: 0,
            n: 3
        },
        body: vec![],
        facing: Direction::East
    };
    let head = grid.contains(&Point {
        x: 0,
        y:0,
        n:3
    });
    if head == false {

        grid[0] = Point {
        x: 0, y: 0, n: 3
        }
    }
    (grid, snake)
}
fn draw_grid(grid: Vec<Point>) {
    println!("{:?}", grid);
    let mut grid_string = String::new();
    for (i,n ) in grid.iter().enumerate() {
        if i % 10 == 0 {
            grid_string.push_str("\n");
        }
        match n.n {
            0 => grid_string = format!("{}⬛", grid_string),
            1 => grid_string = format!("{}🟩", grid_string),
            2 => grid_string = format!("{}🍎", grid_string),
            3 => grid_string = format!("{}🟥", grid_string),
            _ => println!("err")
        }
    }
    println!("{}", grid_string);
}
fn animate(mut grid: Vec<Point>, mut snake: Snake) -> (Vec<Point>, Snake) {
    draw_grid(grid.clone());
    println!("{:?}", snake.head);
    (grid, snake) = move_snake(grid, snake);
    (grid, snake)
}
fn move_snake(mut grid: Vec<Point>, mut snake:Snake) -> (Vec<Point>, Snake) {
    
    snake.body.pop();
snake.body.push(Point {
        x: snake.head.x,
        y: snake.head.y,    
        n: 2,
    });
    match snake.facing {
        Direction::West => snake.head.x -= 1,
        Direction::East => snake.head.x += 1,
        Direction::North => snake.head.y += 1,
        Direction::South => snake.head.y -= 1,
    }
    let position = snake.head.x + snake.head.y * 10;
    grid[(position- 1)as usize] = Point {
        x: snake.head.x - 1,
        y: snake.head.y,
        n: 0,
    };
    grid[position as usize] = snake.clone().head;
    (grid, snake)
}