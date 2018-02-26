# Conway's Game of Life in Rust

[![Build Status](https://travis-ci.org/benjamintanweihao/conway-gol.svg?branch=master)](https://travis-ci.org/benjamintanweihao/conway-gol)

Me learning some Rust. 

## Progress

- [X] Implement the GoL logic, with a text renderer
- [X] Render a grid in SDL
- [X] Implement a [suitable format](http://golly.sourceforge.net/Help/formats.html) to read GoL patterns
- [ ] Implement the [Hashlife algorithm](https://en.wikipedia.org/wiki/Hashlife)

## Rules

1. Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
1. Any live cell with two or three live neighbours lives on to the next generation.
1. Any live cell with more than three live neighbours dies, as if by overpopulation.
1. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.

## Demo

[![](https://i.imgur.com/MhL9mCX.png)](https://www.youtube.com/watch?v=gLmGns2qug0)
