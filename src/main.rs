extern crate sdl2;

use std::{process, vec};
use std::{thread, time};
use std::collections::{VecDeque, HashMap};

use rand::Rng;

use sdl2::{event::Event};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

const MULTIPLIER: u32 = 16;
const LENGTH_TEST: u32 = 32;

fn gen_coordinates(min: &u32, max: &u32) -> (usize, usize) {
    let mut rng = rand::thread_rng();
    (rng.gen_range(0..*min as usize), rng.gen_range(0..*max as usize))
}

fn draw_screen(canvas: &mut Canvas<Window>, grid: &Vec<Vec<u8>>) {
            // Draw changes
        // 1 is part of the snake, 2 is apple
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
                } else if *y_item == 2 {
                    canvas.set_draw_color(Color::RED);
                    canvas.fill_rect(Rect::new(
                        x as i32 * MULTIPLIER as i32,
                        y as i32 * MULTIPLIER as i32, 
                        MULTIPLIER, 
                        MULTIPLIER
                    )).expect("Drawing failed");
                }
            }
        }

}

fn main() {
    // Used to know if the snake is doing a 180
    let mut turn_around: HashMap<u8, u8> = HashMap::new();
    turn_around.insert(0, 2);
    turn_around.insert(2, 0);
    turn_around.insert(1, 3);
    turn_around.insert(3, 1);

    let length = LENGTH_TEST as usize;
    let mut snake_length = 3;
    let mut grid: Vec<Vec<u8>> = vec![vec![0; length]; length];
    let mut head: (usize, usize) = (length / 2, length / 2);
    let mut snake: VecDeque<(usize, usize)> = VecDeque::new();
    let mut direction: u8 = 0;
    let mut prior_direction;
    let mut apple_present = false;
    let mut ready = false;

    let sdl_content = sdl2::init().unwrap();
    let video_subsystem = sdl_content.video().unwrap();
    
    let window = video_subsystem.window(
         "Rusty Snake",
         LENGTH_TEST * MULTIPLIER,
         LENGTH_TEST * MULTIPLIER
        )
        .position_centered()
        .build()
        .unwrap();
        
    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_content.event_pump().unwrap();

    loop {
        thread::sleep(time::Duration::from_millis(100));
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        prior_direction = direction;

        // Key handling
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    process::exit(0);
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    direction = 0;
                    ready = true;
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    direction = 1;
                    ready = true;
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    direction = 2;
                    ready = true;
                }
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    direction = 3;
                    ready = true;
                },
                _ => {}
            }
        }

        // Wait for input in order to start
        if !ready {
            continue;
        }

        if direction == *turn_around.get(&prior_direction).unwrap() {
            direction = prior_direction;    
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
        
        // If out of bounds or snake hits itself, Game Over.
        // If an apple is eaten, increase the length of the snake.
        if head.0 >= 32 || head.1 >= 32 {
            break;
        } else if grid[head.0][head.1] == 1 {
            break;
        } else if grid[head.0][head.1] == 2 {
            snake_length += 1;
            apple_present = false;
        }

        // Generate apple if one is not present.
        if !apple_present {
            let mut apple: (usize, usize);
            loop {
                apple = gen_coordinates(&LENGTH_TEST, &LENGTH_TEST);
                if grid[apple.0][apple.1] != 1 {
                    break;
                }
            }
            grid[apple.0][apple.1] = 2;
            apple_present = true;
        }

        if snake.len() > snake_length {
            let end_tail = snake.pop_front().unwrap();
            grid[end_tail.0][end_tail.1] = 0;
        }

        grid[head.0][head.1] = 1;

        draw_screen(&mut canvas, &grid);

        canvas.present();

        snake.push_back(head);


    }
    println!("Game Over!");
}
