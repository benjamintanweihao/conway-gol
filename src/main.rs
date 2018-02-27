mod gol;

extern crate reqwest;
extern crate sdl2;

use std::io::BufReader;
use std::fs::File;
use gol::world::World;
use gol::gl_renderer::gl_renderer;
use gol::rle_reader::rle_reader;

fn main() {
    // let path = "./res/tm.rle";
    // let f = File::open(path).expect("file not found");
    // let file = BufReader::new(&f);

    let link = "http://www.conwaylife.com/patterns/3enginecordershiprake.rle";
    let resp = reqwest::get(link).unwrap();
    assert!(resp.status().is_success());
    let file = BufReader::new(resp);

    let (size, positions) = rle_reader::read(file);
    let world = World::new(positions, size);
    gl_renderer::render(&world);
}
