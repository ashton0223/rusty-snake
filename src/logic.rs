use std::{process, vec};
use std::{thread, time};
use std::collections::{VecDeque, HashMap};

use rand::Rng;

use sdl2::EventPump;
use sdl2::{event::Event};
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;


fn gen_coordinates(min: &u32, max: &u32) -> (usize, usize) {
    let mut rng = rand::thread_rng();
    (rng.gen_range(0..*min as usize), rng.gen_range(0..*max as usize))
}

pub fn run_text_screen(
    canvas: &mut Canvas<Window>,
    width: u32, 
    text: &str,
    draw_text: fn(canvas: &mut Canvas<Window>, text: &str, window_w: u32), 
) {
    draw_text(canvas, text, width);
}

pub fn run_snake(
    multiplier: u32, 
    length_test: u32, 
    initial_round: bool,
    canvas: &mut Canvas<Window>, 
    event_pump: &mut EventPump,
    clear_screen: fn(canvas: &mut Canvas<Window>),
    draw_screen: fn(canvas: &mut Canvas<Window>, grid: &Vec<Vec<u8>>, multiplier: u32) -> ()
) -> u32 {
    // Used to know if the snake is doing a 180
    let mut turn_around: HashMap<u8, u8> = HashMap::new();
    turn_around.insert(0, 2);
    turn_around.insert(2, 0);
    turn_around.insert(1, 3);
    turn_around.insert(3, 1);

    let length = length_test as usize;
    let mut snake_length = 3;
    let mut grid: Vec<Vec<u8>> = vec![vec![0; length]; length];
    let mut head: (usize, usize) = (length / 2, length / 2);
    let mut snake: VecDeque<(usize, usize)> = VecDeque::new();
    let mut direction: u8 = 0;
    let mut prior_direction;
    let mut apple_present = false;
    let mut ready1 = initial_round;
    let mut ready2 = false;



    loop {
        // Slow the snake down
        thread::sleep(time::Duration::from_millis(100));

        clear_screen(canvas);

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

            // Needs both KeyUp, then KeyDown, then KeyUp to start
            // Prevents accidentally starting a new round too early
            match event {
                Event::KeyUp { .. } => {
                    ready1 = true;
                },
                Event::KeyDown { .. } => {
                    if ready1 {
                        ready2 = true;
                    }
                },
                _ => {}
            }
        }

        // Wait for input in order to start
        if !(ready1 && ready2) {
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
                apple = gen_coordinates(&length_test, &length_test);
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

        draw_screen(canvas, &grid, multiplier);

        canvas.present();

        snake.push_back(head);


    }

    snake_length as u32 - 3
}