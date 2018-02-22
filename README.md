# Conway's Game of Life in Rust

Me learning some Rust. 

## Progress

- [ ] Implement the GoL logic, with a text renderer
- [ ] Render a grid in SDL
- [ ] Populate a grid by clicking on the cells and triggering evolution with <kbd>SPC</kbd>
- [ ] Implement a [suitable format](http://golly.sourceforge.net/Help/formats.html) to read GoL patterns
- [ ] Implement the [Hashlife algorithm](https://en.wikipedia.org/wiki/Hashlife)

## Rules

1. Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
1. Any live cell with two or three live neighbours lives on to the next generation.
1. Any live cell with more than three live neighbours dies, as if by overpopulation.
1. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
