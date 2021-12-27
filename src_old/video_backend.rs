use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::render::TextureQuery;
use sdl2::rwops::RWops;

use super::logic;

// Handle converting to i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

fn get_centered_rect(rect_w: u32, rect_h: u32, window_w: u32, window_h: u32) -> Rect {
    let x = (window_w - rect_w) / 2;
    let y = (window_h - rect_h) / 2;
    rect!(x, y, rect_w, rect_h)
}

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

fn draw_text(canvas: &mut Canvas<Window>, text: &str, window_w: u32) {
    let ttf_context = sdl2::ttf::init().unwrap();
    let texture_creator = canvas.texture_creator();

    let font_file = RWops::from_bytes(
        include_bytes!("../res/font/PressStart2P-Regular.ttf")
    ).unwrap();

    let font = ttf_context.load_font_from_rwops(
        font_file,
        16
        )
        .unwrap();
    
    // render text to a surface and convert to texture
    let surface = font
        .render(text)
        .blended_wrapped(Color::WHITE, 300)
        .unwrap();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();

    let TextureQuery {width, height, ..} = texture.query();

    let centered_rect = get_centered_rect(
        width,
        height,
        window_w,
        window_w
    );

    canvas.copy(&texture, None, Some(centered_rect)).unwrap();
    canvas.present();
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

    logic::run_text_screen(
        &mut canvas, 
        length_test * multiplier, 
        "Welcome to Snake!\n\nUse WASD keys to move", 
        draw_text
    );

    let mut initial_round = true;

    loop {
        let score = logic::run_snake(
            multiplier, 
            length_test, 
            initial_round,
            &mut canvas, 
            &mut event_pump,
            clear_screen,
            draw_screen
        );
        logic::run_text_screen(
            &mut canvas,
            length_test * multiplier,
            format!("Score: {}\n\nPress any key to continue", score).as_str(), 
            draw_text
        );
        
        initial_round = false;
    }
}