fn main() {
    let lines = include_str!("input.txt")
        .lines()
        .map(|l| l.as_bytes())
        .collect::<Vec<&[u8]>>();

    // Get the diagnostic reports first

    let mut info = calculate_info(&lines);
    println!("{:?}", info);

    // Loop through each and remove until one number
    // left

    let (zero, one) = info.remove(0);
    let first_oxygen_common = if one >= zero { b'1' } else { b'0' };

    let (mut oxygen_bits, mut c02_bits): (Vec<_>, Vec<_>) = lines
        .into_iter()
        .partition(|c| *c.first().unwrap() == first_oxygen_common);

    let mut i = 1;
    while oxygen_bits.len() > 1 {
        info = calculate_info(&oxygen_bits);
        let stats = info[i];
        let zero = stats.0;
        let one = stats.1;
        let oxygen_common = if one >= zero { b'1' } else { b'0' };
        oxygen_bits.retain(|c| *c.get(i).unwrap() == oxygen_common);

        i += 1;
    }

    i = 1;
    while c02_bits.len() > 1 {
        info = calculate_info(&c02_bits);
        let stats = info[i];
        let zero = stats.0;
        let one = stats.1;

        let c02_common = if zero <= one { b'0' } else { b'1' };
        c02_bits.retain(|c| *c.get(i).unwrap() == c02_common);
        i += 1;
    }

    let oxygen_binary = std::str::from_utf8(oxygen_bits.first().unwrap()).unwrap();
    let oxygen_decimal = u32::from_str_radix(oxygen_binary, 2).unwrap();

    let c02_binary = std::str::from_utf8(c02_bits.first().unwrap()).unwrap();
    let c02_decimal = u32::from_str_radix(c02_binary, 2).unwrap();

    println!(
        "oxygen: {:?}\nc02: {:?}\nTotal: {}",
        oxygen_decimal,
        c02_decimal,
        oxygen_decimal * c02_decimal
    );
}

fn calculate_info(lines: &Vec<&[u8]>) -> Vec<(u32, u32)> {
    let mut info: Vec<(u32, u32)> = Vec::new();

    for line in lines {
        line.clone()
            .into_iter()
            .enumerate()
            .for_each(|(i, c)| match c {
                b'0' => {
                    if let Some(elem) = info.get_mut(i) {
                        *elem = (elem.0 + 1, elem.1);
                    } else {
                        info.insert(i, (1, 0));
                    }
                }
                b'1' => {
                    if let Some(elem) = info.get_mut(i) {
                        *elem = (elem.0, elem.1 + 1);
                    } else {
                        info.insert(i, (0, 1));
                    }
                }
                _ => unreachable!(),
            });
    }

    info
}
