pub const BORDER_SIZE: i32 = 20;
pub const SIDE_WIDTH: i32 = 300;
pub const CELL_WIDTH: i32 = 10;
pub const CELL_EDGES: i32 = 45;

pub fn calc_logical_index(x: i32, y: i32) -> usize {
    return ((x * CELL_EDGES * 2) + y) as usize;
}

pub fn calc_physical_index(xpos: f64, ypos: f64) -> usize {
    let x = xpos / CELL_WIDTH as f64;
    let y = ypos / CELL_WIDTH as f64;
    return calc_logical_index(x as i32, y as i32);
}
