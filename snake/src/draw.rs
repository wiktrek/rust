use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;
const BLOCK_SIZE: f64 = 25.0;
pub fn to_coord(game_coord: i32) -> f64 {
    let coord = (game_coord as f64) * BLOCK_SIZE;
return coord;
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
let gui_x = to_coord(x);
let gui_y = to_coord(y);
rectangle(
    color,
    [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
    con.transform,
    g,
)
}
pub fn draw_rectangle(color: Color, x: i32, y: i32, width: i32, height: i32, con: &Context, g: &mut G2d) {
    let x = to_coord(x);
    let y = to_coord(y);
    
}