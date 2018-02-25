mod gol;

extern crate sdl2;

use gol::world::{Position, World};
use gol::gl_renderer::gl_renderer;
use gol::rle_reader::rle_reader;

fn main() {
    let size = 50;
    let positions = rle_reader::read("./res/twogun.lif");
    let world = World::new(positions, size);
    gl_renderer::render(&world);
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
