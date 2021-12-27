use graphics::{Context, Graphics};

use crate::game_controller::GameController;

pub struct GameView {
    // Can I do nothing here?
}

impl GameView {
    pub fn new() -> GameView {
        GameView {}
    }

    pub fn draw<G: Graphics>(controller: &GameController, c: &Context, g: &mut G) {
        
    }
}