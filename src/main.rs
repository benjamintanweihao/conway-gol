mod gol;

use gol::cell::Cell;

fn main() {
    let cell = Cell::new(0, 0, true);

    println!("The cell is at {:#?}", cell);
}
