extern crate sdl2;

mod gol;

use std::{thread, time};
use gol::world::{Position, World, SIZE};

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const CELL_SIZE: u32 = 4;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Conway's Game of Life", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let duration = time::Duration::from_millis(50);
    let mut world = World::new(combination());

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        world = world.tick();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let cell_color = Color::RGB(0, 255, 0);
        canvas.set_draw_color(cell_color);

        for y in 0..SIZE {
            for x in 0..SIZE {
                if world.is_alive((x, y)) == true {
                    let _ = canvas.fill_rect(Rect::new(
                        (x as i32) * (CELL_SIZE as i32),
                        (y as i32) * (CELL_SIZE as i32),
                        CELL_SIZE - 1,
                        CELL_SIZE - 1,
                    ));
                }
            }
        }

        canvas.present();

        thread::sleep(duration);
    }
}
#[allow(dead_code)]
fn combination() -> Vec<Position> {
    vec![
        (0, 12),
        (1, 12),
        (2, 12),
        (1, 6),
        (2, 7),
        (0, 8),
        (1, 8),
        (2, 8),
    ]
}

#[allow(dead_code)]
fn glider() -> Vec<Position> {
    vec![(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)]
}
