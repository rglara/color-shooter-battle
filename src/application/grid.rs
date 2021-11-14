pub struct Grid {
    pub cells: [i8; (super::common::CELL_EDGES * super::common::CELL_EDGES * 4) as usize],
}

impl Grid {
    pub fn new() -> Grid {
        let mut c = [0; (super::common::CELL_EDGES * super::common::CELL_EDGES * 4) as usize];
        for y in 0..(super::common::CELL_EDGES * 2) {
            for x in 0..(super::common::CELL_EDGES * 2) {
                let index = super::common::calc_logical_index(x, y);
                if x < super::common::CELL_EDGES && y < super::common::CELL_EDGES {
                    c[index] = 1;
                } else if x >= super::common::CELL_EDGES && y < super::common::CELL_EDGES {
                    c[index] = 2;
                } else if x < super::common::CELL_EDGES && y >= super::common::CELL_EDGES {
                    c[index] = 3;
                } else if x >= super::common::CELL_EDGES && y >= super::common::CELL_EDGES {
                    c[index] = 4;
                }
            }
        }
        Grid { cells: c }
    }

    pub fn check_collision(&mut self, x: f64, y: f64, cannon_id: i8) -> bool {
        let index = super::common::calc_physical_index(x, y);
        if self.cells[index] != cannon_id {
            self.cells[index] = cannon_id;
            return true;
        }
        return false;
    }
}
