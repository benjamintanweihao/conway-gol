type Grid = Vec<Vec<bool>>;
type Position = (usize, usize);

pub const SIZE: usize = 50;

#[derive(Debug)]
pub struct World {
    grid: Grid,
}

impl World {
    pub fn new(positions: Vec<Position>) -> World {
        World {
            grid: World::make_grid(positions.clone()),
        }
    }

    pub fn is_alive(&self, position: Position) -> bool {
        let (x, y) = position;
        self.grid[x][y] == true
    }

    fn make_grid(positions: Vec<Position>) -> Grid {
        let mut grid = Vec::with_capacity(SIZE);

        for _y in 0..SIZE {
            let mut row = Vec::with_capacity(SIZE);
            for _x in 0..SIZE {
                row.push(false);
            }
            grid.push(row);
        }

        for &(x, y) in positions.iter() {
            grid[x][y] = true;
        }

        grid
    }

    pub fn tick(&self) -> World {
        let mut new_grid = self.grid.clone();

        for y in 0..SIZE {
            for x in 0..SIZE {
                if new_grid[x][y] == true {
                    match self.count_neighbours((x, y)) {
                        0...1 => new_grid[x][y] = false,
                        2 | 3 => new_grid[x][y] = true,
                        4...8 => new_grid[x][y] = false,
                        _ => (),
                    }
                } else {
                    if self.count_neighbours((x, y)) == 3 {
                        new_grid[x][y] = true
                    }
                }
            }
        }

        World { grid: new_grid }
    }

    fn count_neighbours(&self, position: Position) -> i32 {
        fn dec(n: usize) -> usize {
            if (n as i32) - 1 < 0 {
                SIZE - 1
            } else {
                n - 1
            }
        }

        fn inc(n: usize) -> usize {
            if n + 1 == SIZE {
                0
            } else {
                n + 1
            }
        }

        let grid = &self.grid;
        let (x, y) = position;

        return (grid[dec(x)][dec(y)] as i32) + (grid[x][dec(y)] as i32)
            + (grid[inc(x)][dec(y)] as i32) + (grid[dec(x)][y] as i32)
            + (grid[inc(x)][y] as i32) + (grid[dec(x)][inc(y)] as i32)
            + (grid[x][inc(y)] as i32) + (grid[inc(x)][inc(y)] as i32);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_world_has_no_neighbors() {
        let world = World::new(Vec::new());

        assert_eq!(0, world.count_neighbours((0, 0)));
    }

    #[test]
    fn counting_with_neighbors() {
        let positions = vec![(0, 0), (1, 0), (2, 0)];
        let world = World::new(positions);

        assert_eq!(3, world.count_neighbours((1, 1)));
        assert_eq!(2, world.count_neighbours((2, 1)));
    }
}
