use std::convert::TryInto;
use std::fmt::Debug;

fn main() {
    let coordinates = parse();

    let (width, height) = get_width_and_height_from(&coordinates);

    let mut boards = vec![vec![0; (width).into()]; (height).into()];

    for pair in &coordinates {
        let line_coordinates = pair.line_coordinates();

        for c in line_coordinates {
            let row = boards.get_mut(c.1 as usize).unwrap();
            let col = row.get_mut(c.0 as usize).unwrap();

            *col += 1;
        }
    }

    for row in &boards {
        for col in row {
            if *col == 0 {
                print!(".");
            } else {
                print!("{}", col);
            }
        }

        println!("");
    }
    println!("");

    let total: usize = boards.iter().flatten().filter(|&&x| x > 1).count();
    println!("Total: {}", total);
}

type Coordinate = (i16, i16);

struct PairCoordinates {
    first: Coordinate,
    second: Coordinate,
}

impl Debug for PairCoordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{} -> {},{}",
            self.first.0, self.first.1, self.second.0, self.second.1
        )
    }
}

impl PairCoordinates {
    fn line_coordinates(&self) -> Vec<Coordinate> {
        let (larger_y, smaller_y) = if self.first.1 > self.second.1 {
            (self.first.1, self.second.1)
        } else {
            (self.second.1, self.first.1)
        };

        let (larger_x, smaller_x) = if self.first.0 > self.second.0 {
            (self.first.0, self.second.0)
        } else {
            (self.second.0, self.first.0)
        };

        if self.first.0 == self.second.0 {
            (smaller_y..=larger_y).map(|y| (self.first.0, y)).collect()
        } else if self.first.1 == self.second.1 {
            (smaller_x..=larger_x).map(|x| (x, self.first.1)).collect()
        } else if larger_x - smaller_x == larger_y - smaller_y {
            let (right, left) = if self.first.1 < self.second.1 {
                (self.first, self.second)
            } else {
                (self.second, self.first)
            };

            let mut coordinates = vec![right];

            // This implementation is so... unsafe.
            let move_x = if (left.0 - right.0) > 0 { 1 } else { -1 };
            let move_y = if (left.1 - right.1) > 0 { 1 } else { -1 };

            let mut x = right.0;
            let mut y = right.1;

            while x != left.0 && y != left.1 {
                x += move_x;
                y += move_y;
                coordinates.push((x.try_into().unwrap(), y.try_into().unwrap()));
            }

            coordinates
        } else {
            Vec::new()
        }
    }
}

fn parse() -> Vec<PairCoordinates> {
    include_str!("input.txt")
        .lines()
        .map(|l| {
            let (first, second) = l.split_once(" -> ").unwrap();

            PairCoordinates {
                first: parse_coordinate(&first),
                second: parse_coordinate(&second),
            }
        })
        .collect()
}

fn parse_coordinate(line: &str) -> Coordinate {
    let (x, y) = line.split_once(",").unwrap();

    (x.parse().unwrap(), y.parse().unwrap())
}

fn get_width_and_height_from(coordinates: &Vec<PairCoordinates>) -> (usize, usize) {
    let (mut width, mut height) =
        coordinates
            .iter()
            .fold((0, 0), |(mut width, mut height), coordinate| {
                if coordinate.first.0 >= width {
                    width = coordinate.first.0;
                }

                if coordinate.second.0 >= width {
                    width = coordinate.second.0;
                }

                if coordinate.first.1 >= height {
                    height = coordinate.first.1;
                }

                if coordinate.second.1 >= height {
                    height = coordinate.second.1;
                }

                (width, height)
            });

    width += 1;
    height += 1;

    (width.try_into().unwrap(), height.try_into().unwrap())
}
