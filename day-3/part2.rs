use std::str::Chars;

fn main() {
    let lines = include_str!("sample.txt")
        .lines()
        .map(|l| l.chars())
        .collect::<Vec<_>>();
    let mut info: Vec<(u32, u32)> = Vec::new();

    // Get the diagnostic reports first
    for line in &lines {
        println!("{:?}", line);
        line.clone().enumerate().for_each(|(i, c)| match c {
            '0' => {
                if let Some(elem) = info.get_mut(i) {
                    *elem = (elem.0 + 1, elem.1);
                } else {
                    info.insert(i, (1, 0));
                }
            }
            '1' => {
                if let Some(elem) = info.get_mut(i) {
                    *elem = (elem.0, elem.1 + 1);
                } else {
                    info.insert(i, (0, 1));
                }
            }
            _ => unreachable!(),
        });
    }

    println!("{:?}", info);

    // Loop through each and remove until one number
    // left

    let (zero, one) = info.remove(0);
    let first_oxygen_common = if one >= zero { '1' } else { '0' };

    let (mut oxygen_bits, mut c02_bits): (Vec<&Chars<'_>>, Vec<&Chars<'_>>) = lines
        .iter()
        .partition(|c| c.clone().nth(0) == Some(first_oxygen_common));

    println!("{:?}\n{:?}\n", oxygen_bits, c02_bits);

    info.into_iter().enumerate().for_each(|(i, (zero, one))| {
        let oxygen_common = if one >= zero { '1' } else { '0' };
        let c02_common = if zero <= one { '0' } else { '1' };

        if oxygen_bits.len() != 1 {
            oxygen_bits = oxygen_bits
                .iter()
                .filter(|&&c| c.nth(0).unwrap() == oxygen_common)
                .collect::<Vec<_>>();
        }

        if c02_bits.len() != 1 {
            c02_bits = c02_bits
                .iter()
                .filter(|&&c| c.nth(0).unwrap() == c02_common)
                .collect::<Vec<_>>();
        }
    });

    println!("{:?}\n{:?}\n", oxygen_bits, c02_bits);
}
