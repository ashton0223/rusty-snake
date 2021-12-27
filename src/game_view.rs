const WIDTH: usize = 16;
const RES_WIDTH: usize = 512;

use graphics::types::Color;
use graphics::{Context, Graphics};
use graphics::Rectangle;

use crate::game_controller::GameController;

pub struct GameView {
    // Can I do nothing here?
}

impl GameView {
    pub fn new() -> GameView {
        GameView {}
    }

    pub fn draw<G: Graphics>(&self, controller: &GameController, c: &Context, g: &mut G) {
        for x in 0..WIDTH {
            for y in 0..WIDTH {
                if let Some(square) = controller.game.square_type([x, y]) {
                    let ratio = (RES_WIDTH / WIDTH) as f64;
                    let screen_pos = [
                        x as f64 * ratio,
                        y as f64 * ratio
                    ];
                    let square_data = [
                        screen_pos[0], screen_pos[1],
                        ratio, ratio,
                    ];
                    let color = [1.0; 4];
                    Rectangle::new(color)
                        .draw(square_data, &c.draw_state, c.transform, g);
                }
            }
        }
    }
}