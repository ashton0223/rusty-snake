use piston::input::GenericEvent;

use crate::Game;

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
            println!("{:?}", pos);
        }
    }
}