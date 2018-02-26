pub mod rle_reader {
    use std::cmp;
    use std::io::BufReader;
    use std::io::BufRead;
    use std::fs::File;
    use gol::world::Position;

    pub fn read(path: &str) -> (usize, Vec<Position>) {
        let f = File::open(path).expect("file not found");
        let file = BufReader::new(&f);
        let mut positions = Vec::new();

        let mut x = 0;
        let mut y = 0;
        let mut size = 0;

        'outer: for (n, line) in file.lines().enumerate() {
            let l = line.unwrap();

            if n == 0 {
                size = parse_header_for_size(l);
            } else {
                let mut num_chars = String::from(""); // used to track numbers
                for c in l.chars() {
                    if c == '!' {
                        break 'outer;
                    } else {
                        match c {
                            '$' => {
                                if num_chars != "" {
                                    let num = num_chars.parse::<i32>().unwrap();
                                    x = 0;
                                    y += num;
                                } else {
                                    x = 0;
                                    y += 1;
                                }
                                num_chars = String::from("");
                            }
                            'o' => {
                                if num_chars != "" {
                                    let num = num_chars.parse::<i32>().unwrap();
                                    // Create number of o's
                                    for n in 0..num {
                                        positions.push(((x + n) as usize, y as usize));
                                    }

                                    x += num;
                                    num_chars = String::from("");
                                } else {
                                    // Just one 'o
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

        return (size as usize, positions);
    }

    fn parse_header_for_size(line: String) -> usize {
        let l1 = line.replace(" ", "");
        let l2 = l1.split(',').collect::<Vec<_>>();
        let l3 = &l2[0..2];
        let l4 = l3.iter()
            .flat_map(|&x| x.split('=').collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let x = l4[1].parse::<i32>().unwrap();
        let y = l4[3].parse::<i32>().unwrap();

        cmp::max(x, y) as usize
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_glider() {
        let expected = vec![(0, 0), (1, 0), (2, 0), (2, 1), (1, 2)];
        let actual = rle_reader::read("./res/glider.rle");

        assert_eq!(expected, actual)
    }
}
