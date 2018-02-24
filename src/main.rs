extern crate sdl2;

mod gol;

use std::{thread, time};
use gol::world::{Position, World};
use gol::text_renderer::text_renderer;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

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

    let cell_color = Color::RGB(0, 255, 0);
    canvas.set_draw_color(cell_color);
    canvas.fill_rect(Rect::new(10, 10, 10, 10));
    canvas.fill_rect(Rect::new(0, 0, 10, 10));
    canvas.fill_rect(Rect::new(20, 20, 10, 10));
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

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
    }

    // let world = World::new(combination());

    // let duration = time::Duration::from_millis(100);
    // let mut world = world;
    // loop {
    //     text_renderer::render(&world);
    //     world = world.tick();
    //     thread::sleep(duration);
    // }
}

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

fn glider() -> Vec<Position> {
    vec![(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)]
}
