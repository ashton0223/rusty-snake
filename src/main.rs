#![windows_subsystem = "windows"]

extern crate sdl2;

mod video_backend;
mod logic;

const MULTIPLIER: u32 = 16;
const LENGTH_TEST: u32 = 32;
const TITLE: &str = "Rusty Snake";

fn main() {
    video_backend::start_gfx(MULTIPLIER, LENGTH_TEST, TITLE);
}

