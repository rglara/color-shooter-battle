pub const BORDER_SIZE: i32 = 20;
pub const SIDE_WIDTH: i32 = 300;
pub const CELL_WIDTH: i32 = 14;
pub const CELL_EDGES: i32 = 32;

pub fn calc_logical_index(x: i32, y: i32) -> usize {
    return ((x * CELL_EDGES * 2) + y) as usize;
}

pub fn calc_physical_index(xpos: f64, ypos: f64) -> usize {
    let x = xpos / CELL_WIDTH as f64;
    let y = ypos / CELL_WIDTH as f64;
    return calc_logical_index(x as i32, y as i32);
}

pub fn check_circle_boundary_collisions(
    is_inner: bool,
    boundaries: [f64; 4],
    position: [f64; 2],
    radius: f64,
    angle: f64,
) -> f64 {
    let mut new_angle = angle;
    if is_inner {
        if ((position[0] - radius) < boundaries[0])
            || ((position[0] + radius) > (boundaries[0] + boundaries[2]))
        {
            new_angle = 180.0 - new_angle;
        }
        if ((position[1] - radius) < boundaries[1])
            || ((position[1] + radius) > (boundaries[1] + boundaries[3]))
        {
            new_angle = 360.0 - new_angle;
        }
    } else {
        if position[0] < boundaries[0] {
            new_angle = 180.0 - new_angle;
        }
        if position[0] > (boundaries[0] + boundaries[2]) {
            new_angle = 180.0 - new_angle;
        }
        if position[1] < boundaries[1] {
            new_angle = 360.0 - new_angle;
        }
        if position[1] > (boundaries[1] + boundaries[3]) {
            new_angle = 360.0 - new_angle;
        }
        println!("old: {} | new: {} => (x{}, y{}) r{} [x{}, y{}, w{}, h{}]", angle, new_angle, position[0], position[1], radius, boundaries[0], boundaries[1], boundaries[2], boundaries[3]);
    }
    new_angle
}
