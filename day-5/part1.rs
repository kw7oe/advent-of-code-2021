use std::fmt::Debug;

fn main() {
    let coordinates = parse();

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

type Coordinate = (u16, u16);

struct PairCoordinates {
    first: Coordinate,
    second: Coordinate,
}

impl Debug for PairCoordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{} -> {}.{}",
            self.first.0, self.first.1, self.second.0, self.second.1
        )
    }
}

impl PairCoordinates {
    fn line_coordinates(&self) -> Vec<Coordinate> {
        if self.first.0 == self.second.0 {
            let (larger, smaller) = if self.first.1 > self.second.1 {
                (self.first.1, self.second.1)
            } else {
                (self.second.1, self.first.1)
            };

            (smaller..=larger).map(|y| (self.first.0, y)).collect()
        } else if self.first.1 == self.second.1 {
            let (larger, smaller) = if self.first.0 > self.second.0 {
                (self.first.0, self.second.0)
            } else {
                (self.second.0, self.first.0)
            };

            (smaller..=larger).map(|x| (x, self.first.1)).collect()
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
