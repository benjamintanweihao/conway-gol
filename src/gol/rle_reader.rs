mod rle_reader {
    use std::io::BufReader;
    use std::io::BufRead;
    use std::fs::File;
    use gol::world::Position;

    pub fn read(path: &str) -> Vec<Position> {
        let f = File::open(path).expect("file not found");
        let file = BufReader::new(&f);
        let mut positions = Vec::new();

        'outer: for (n, line) in file.lines().enumerate() {
            let l = line.unwrap();
            if n == 0 {
                // Read header
                println!("Header: {}", l);
            } else {
                let mut num_chars = String::from(""); // used to track numbers
                let mut x = 0;
                let mut y = 0;

                for c in l.chars() {
                    if c == '!' {
                        break 'outer;
                    } else {
                        match c {
                            '$' => {
                                x = 0;
                                y += 1;
                                num_chars = String::from("");
                            }
                            'o' => {
                                if num_chars != "" {
                                    let num = num_chars.parse::<i32>().unwrap();
                                    // Create number of o's
                                    for n in 0..num {
                                        println!("({}, {})", x + n, y);
                                        positions.push(((x + n) as usize , y as usize));
                                    }

                                    x += num;
                                    num_chars = String::from("");
                                } else {
                                    // Just one 'o
                                    println!("({}, {})", x, y);
                                    positions.push((x as usize, y as usize));
                                    x += 1;
                                }
                            }
                            'b' => {
                                if num_chars != "" {
                                    let num = num_chars.parse::<i32>().unwrap();
                                    x += num;
                                    num_chars = String::from("");
                                } else {
                                    // Just one 'b
                                    x += 1;
                                }
                            }
                            n => num_chars.push(n),
                        }
                    }
                }
            }
        }

        return positions;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_glider() {
        // rle_reader::read("./res/tm.lif");
        let expected = vec![(0, 0), (1, 0), (2, 0), (2, 1), (1, 2)];
        let actual = rle_reader::read("./res/glider.lif");

        assert_eq!(expected, actual)
    }
}
