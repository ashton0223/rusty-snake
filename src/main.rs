//#![deny(missing_docs)] // Forced documentation :)

//! Test thing

extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::{Events, EventLoop, EventSettings};
use piston::input::RenderEvent;
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, GlGraphics};

pub use crate::game::Game;
pub use crate::game_controller::GameController;
pub use crate::game_view::{GameView};

mod game;
mod game_controller;
mod game_view;

const RES_WIDTH: usize = 512;

fn main() {
    
    // Tell the window backend what OpenGL version to use
    let opengl = OpenGL::V3_2;
    // Settinsg for new window
    let settings = WindowSettings::new("rusty-snake", [RES_WIDTH as u32; 2])
        .graphics_api(opengl) // Set graphics API
        .exit_on_esc(true);

    // Actual new window
    let mut window: GlutinWindow = settings.build()
        .expect("Could not create window");

    // Setup events for loop
    let mut events = Events::new(EventSettings::new().lazy(true));
    // Shaders/buffer information for OpenGL to talk to GPU
    let mut gl = GlGraphics::new(opengl);

    // Idk random game stuff
    
    let game = Game::new();
    let mut game_controller = GameController::new(game);
    let game_view = GameView::new();

    // Actual event loop?
    while let Some(e) = events.next(&mut window) {
        // Pass events to game controller?
        
        game_controller.event(&e);
        // Event loop emits render event
        if let Some(args) = e.render_args() {
            /*  Inside the render if let block, we call a method on the gl 
                object to create a graphics::Context and a graphics backend 
                implementing the graphics::Graphics trait. */
            gl.draw(args.viewport(), |c, g| {
                // No idea to be honest
                use graphics::{clear};

                // Somehow makes the screen white?
                clear([0.0; 4], g);
                // Renders the game (somehow)
                game_view.draw(&game_controller, &c, g);
            });
        }
    }
}