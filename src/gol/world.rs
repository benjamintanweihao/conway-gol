pub type Position = (usize, usize);
type Grid = Vec<Vec<bool>>;
type Size = usize;

#[derive(Debug)]
pub struct World {
    grid: Grid,
    pub size: Size,
}

impl World {
    pub fn new(positions: Vec<Position>, size: Size) -> World {
        World {
            grid: World::make_grid(positions.clone(), size),
            size: size,
        }
    }

    pub fn is_alive(&self, position: Position) -> bool {
        let (x, y) = position;
        self.grid[x][y] == true
    }

    fn make_grid(positions: Vec<Position>, size: Size) -> Grid {
        let mut grid = Vec::with_capacity(size);

        for _y in 0..size {
            let mut row = Vec::with_capacity(size);
            for _x in 0..size {
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
        let size = self.size;

        for y in 0..size {
            for x in 0..size {
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

        World {
            grid: new_grid,
            size: size,
        }
    }

    fn dec(&self, n: usize) -> usize {
        if (n as i32) - 1 < 0 {
            self.size - 1
        } else {
            n - 1
        }
    }

    fn inc(&self, n: usize) -> usize {
        if n + 1 == self.size {
            0
        } else {
            n + 1
        }
    }

    fn count_neighbours(&self, position: Position) -> i32 {
        let grid = &self.grid;
        let (x, y) = position;

        return (grid[self.dec(x)][self.dec(y)] as i32) + (grid[x][self.dec(y)] as i32)
            + (grid[self.inc(x)][self.dec(y)] as i32) + (grid[self.dec(x)][y] as i32)
            + (grid[self.inc(x)][y] as i32) + (grid[self.dec(x)][self.inc(y)] as i32)
            + (grid[x][self.inc(y)] as i32)
            + (grid[self.inc(x)][self.inc(y)] as i32);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_world_has_no_neighbors() {
        let world = World::new(Vec::new(), 50);

        assert_eq!(0, world.count_neighbours((0, 0)));
    }

    #[test]
    fn counting_with_neighbors() {
        let positions = vec![(0, 0), (1, 0), (2, 0)];
        let world = World::new(positions, 50);

        assert_eq!(3, world.count_neighbours((1, 1)));
        assert_eq!(2, world.count_neighbours((2, 1)));
    }
}
