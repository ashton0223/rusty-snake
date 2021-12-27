use piston::input::GenericEvent;

use crate::Game;

const RES_WIDTH: usize = 512;
const WIDTH: usize = 16;

pub struct GameController {
    pub game: Game,
    pub head: Option<[usize; 2]>
}

impl GameController {
    pub fn new(game: Game) -> GameController {
        GameController {
            game: game,
            head: None,
        }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(pos) = e.mouse_cursor_args() {
            let ratio = RES_WIDTH / WIDTH;
            let x = pos[0] as usize / ratio;
            let y = pos[1] as usize / ratio;
            self.game.set([x, y], 1);
        }
    }
}