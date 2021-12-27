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
}