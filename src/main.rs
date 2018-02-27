mod gol;

extern crate clap;
extern crate reqwest;
extern crate sdl2;

use std::io::BufReader;
use std::fs::File;
use gol::world::World;
use gol::gl_renderer::gl_renderer;
use gol::rle_reader::rle_reader;
use clap::{App, Arg};

fn main() {
    let matches = App::new("Conway's Game of Life")
        .arg(
            Arg::with_name("file")
                .help("the file path of the RLE file")
                .takes_value(true)
                .short("f")
                .multiple(false)
                .required(true)
                .conflicts_with("url"),
        )
        .arg(
            Arg::with_name("url")
                .help("the URL of the RLE file")
                .takes_value(true)
                .short("u")
                .multiple(false)
                .required(true)
                .conflicts_with("file"),
        )
        .get_matches();

    if matches.is_present("file") {
        if let Some(path) = matches.value_of("file") {
            let f = File::open(path).expect("file not found");
            let file = BufReader::new(&f);
            let (size, positions) = rle_reader::read(file);
            let world = World::new(positions, size);
            gl_renderer::render(&world);
        }
    }

    if matches.is_present("url") {
        if let Some(url) = matches.value_of("url") {
            let resp = reqwest::get(url).unwrap();
            assert!(resp.status().is_success());
            let file = BufReader::new(resp);
            let (size, positions) = rle_reader::read(file);
            let world = World::new(positions, size);
            gl_renderer::render(&world);
        }
    }
}
