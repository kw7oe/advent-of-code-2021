use std::collections::HashMap;
fn main() {
    let total = include_str!("input.txt").lines().fold(0, |acc, string| {
        let (signal, output) = string.split_once(" | ").unwrap();

        let digits = process_signal(&signal, &output);

        acc + digits
    });

    println!("Total: {}", total);
}

fn process_signal(signals: &str, output: &str) -> u64 {
    let mut map = HashMap::new();
    map.insert(2, 1);
    map.insert(3, 7);
    map.insert(4, 4);
    map.insert(7, 8);
    let outputs_vec = output.trim().split(" ").collect::<Vec<&str>>();

    let mut right = Vec::new();
    let mut top = None;
    let mut mid_or_top_left = Vec::new();
    let mut bottom_or_bottom_left = Vec::new();
    let mut signals_vec = signals.trim().split(" ").collect::<Vec<&str>>();
    signals_vec.sort_by_key(|a| a.len());

    signals_vec.retain(|c| match c.len() {
        2 => {
            right = c.split("").collect();
            false
        }
        3 => {
            for a in c.split("") {
                if !right.contains(&a) {
                    top = Some(a);
                }
            }

            false
        }
        4 => {
            for a in c.split("") {
                if !right.contains(&a) {
                    mid_or_top_left.push(a);
                }
            }
            false
        }
        7 => {
            for a in c.split("") {
                if !(top == Some(a) || right.contains(&a) || mid_or_top_left.contains(&a)) {
                    bottom_or_bottom_left.push(a);
                }
            }
            false
        }
        _ => true,
    });

    let mut sum = 0;
    let mut pos = 4;

    for word in outputs_vec {
        pos -= 1;
        let len = word.len();

        let digit = match len {
            2 | 3 | 4 | 7 => map.get(&len).unwrap(),
            // Possible answer: 2, 3, 5
            //   - All have top and bottom.
            //   - Only 5 require both middle_or_top_left;
            //   - Only 2 require both bottom_or_bottom_left;
            //   - else is 3
            5 => {
                let mut bottom_or_bottom_left_count = 0;
                let mut middle_or_top_left_count = 0;

                for c in word.split("") {
                    if c != "" && bottom_or_bottom_left.contains(&c) {
                        bottom_or_bottom_left_count += 1;
                    }
                    if c != "" && mid_or_top_left.contains(&c) {
                        middle_or_top_left_count += 1;
                    }
                }

                if bottom_or_bottom_left_count == 2 {
                    &2
                } else if middle_or_top_left_count == 2 {
                    &5
                } else {
                    &3
                }
            }
            // Possible answer: 0, 6, 9
            //   - All have top and bottom.
            //   - 0 doesn't require middle. both 6, 9 require middle_or_top_left;
            //   - only 6 require bottom_or_bottom_left
            //   - else is 9
            6 => {
                let mut middle_or_top_left_count = 0;
                let mut bottom_or_bottom_left_count = 0;

                for c in word.split("") {
                    if c != "" && bottom_or_bottom_left.contains(&c) {
                        bottom_or_bottom_left_count += 1;
                    }
                    if c != "" && mid_or_top_left.contains(&c) {
                        middle_or_top_left_count += 1;
                    }
                }

                // The order of this logic is important.
                if middle_or_top_left_count != 2 {
                    &0
                } else if bottom_or_bottom_left_count == 2 {
                    &6
                } else {
                    &9
                }
            }
            _ => unreachable!(),
        };

        sum += digit * (10_u64.pow(pos));
    }

    sum
}
