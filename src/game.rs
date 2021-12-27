const WIDTH: usize = 16;

pub struct Game {
    pub grid: [[u8; WIDTH]; WIDTH],
}

impl Game {
    pub fn new() -> Game {
        Game {
            grid: [[0; WIDTH]; WIDTH],
        }
    }

    pub fn square_type(&self, pos: [usize; 2]) -> Option<u8> {
        Some(match self.grid[pos[0]][pos[1]] {
            1 => 1,
            _ => return None,
        })
    }

    pub fn set(&mut self, pos: [usize; 2], value: u8) {
        self.grid[pos[0]][pos[1]] = value;
    }
}