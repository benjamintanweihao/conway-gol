#[derive(Debug)]
pub struct Cell {
    x: i32,
    y: i32,
}

impl Cell {
    pub fn new(x: i32, y: i32) -> Cell {
        Cell { x: x, y: y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_cell() {
        let cell = Cell::new(-4, 2);

        assert_eq!(-4, cell.x);
        assert_eq!(2, cell.y);
    }
}
