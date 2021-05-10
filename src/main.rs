extern crate sdl2;

use std::{process, vec};
use std::{thread, time};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

const MULTIPLIER: u32 = 16;
const LENGTH_TEST: u32 = 32;

fn main() {
    let length = LENGTH_TEST as usize;
    let mut grid: Vec<Vec<u8>> = vec![vec![0; length]; length];
    let mut head: (usize, usize) = (length / 2, length / 2);
    let mut direction: u8 = 0;

    let sdl_content = sdl2::init().unwrap();
    let video_subsystem = sdl_content.video().unwrap();
    
    let window = video_subsystem.window(
         "CHIP-8 Interpreter",
         LENGTH_TEST * MULTIPLIER,
         LENGTH_TEST * MULTIPLIER
        )
        .position_centered()
        .build()
        .unwrap();
        
    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_content.event_pump().unwrap();

    loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Key handling
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    process::exit(0);
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    direction = 0;
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    direction = 1;
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    direction = 2;
                }
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    direction = 3;
                },
                _ => {}
            }
        }

        match direction {
            0 => {
                head.0 += 1;
            },
            1 => {
                head.1 += 1;
            },
            2 => {
                head.0 = head.0.wrapping_sub(1);
            },
            3 => {
                head.1 = head.1.wrapping_sub(1);
            },
            _ => {}
        }

        // If out of bounds, Game Over
        if head.0 >= 32 || head.1 >= 32 {
            break;
        }

        grid[head.0][head.1] = 1;

        // Draw changes
        for (x, x_item) in grid.iter().enumerate() {
            for (y, y_item) in x_item.iter().enumerate() {
                if *y_item == 1 {
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                    canvas.fill_rect(Rect::new(
                        x as i32 * MULTIPLIER as i32,
                        y as i32 * MULTIPLIER as i32, 
                        MULTIPLIER, 
                        MULTIPLIER
                    )).expect("Drawing failed");
                }
            }
        }

        canvas.present();

        thread::sleep(time::Duration::from_millis(100));
    }
    println!("Game Over!");
}
