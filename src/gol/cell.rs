#[derive(Debug, PartialEq)]
pub struct Cell {
    x: i32,
    y: i32,
    alive: bool,
}

impl Cell {
    pub fn new(x: i32, y: i32, alive: bool) -> Cell {
        Cell {
            x: x,
            y: y,
            alive: alive,
        }
    }

    pub fn alive(&self) -> bool {
        self.alive
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_cell() {
        let cell = Cell::new(-4, 2, true);

        assert_eq!(-4, cell.x);
        assert_eq!(2, cell.y);
        assert_eq!(true, cell.alive);
    }

    #[test]
    fn equality() {
        let first = Cell::new(-4, 2, true);
        let second = Cell::new(-4, 2, true);

        assert_eq!(first, first);
        assert_eq!(first, second);
        assert_eq!(second, first);
    }

    #[test]
    fn inequality() {
        let first = Cell::new(2, -4, true);
        let second = Cell::new(-4, 2, true);
        let third = Cell::new(-4, 2, false);

        assert_ne!(first, second);
        assert_ne!(second, third);
    }

    #[test]
    fn liveness() {
        let alive = Cell::new(0, 0, true);
        let dead = Cell::new(0, 0, false);

        assert!(alive.alive());
        assert!(!dead.alive());
    }
}
