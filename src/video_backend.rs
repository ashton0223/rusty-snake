use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use super::logic;

fn draw_screen(canvas: &mut Canvas<Window>, grid: &Vec<Vec<u8>>, multiplier: u32) {
            // Draw changes
        // 1 is part of the snake, 2 is apple
        for (x, x_item) in grid.iter().enumerate() {
            for (y, y_item) in x_item.iter().enumerate() {
                if *y_item == 1 {
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                    canvas.fill_rect(Rect::new(
                        x as i32 * multiplier as i32,
                        y as i32 * multiplier as i32, 
                        multiplier, 
                        multiplier
                    )).expect("Drawing failed");
                } else if *y_item == 2 {
                    canvas.set_draw_color(Color::RED);
                    canvas.fill_rect(Rect::new(
                        x as i32 * multiplier as i32,
                        y as i32 * multiplier as i32, 
                        multiplier, 
                        multiplier
                    )).expect("Drawing failed");
                }
            }
        }

}

fn clear_screen(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
}

pub fn start_gfx(multiplier: u32, length_test: u32, title: &str) {
    let sdl_content = sdl2::init().unwrap();
    let video_subsystem = sdl_content.video().unwrap();
    
    let window = video_subsystem.window(
         title,
         length_test * multiplier,
         length_test * multiplier
        )
        .position_centered()
        .build()
        .unwrap();
        
    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_content.event_pump().unwrap();
    
    logic::run_snake(
        multiplier, 
        length_test, 
        &mut canvas, 
        &mut event_pump,
        clear_screen,
        draw_screen
    )
}