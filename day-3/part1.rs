fn main() {
    let (most_sig_bits, least_sig_bits) = include_str!("input.txt")
        .lines()
        .map(|l| l.chars())
        .fold(Vec::new(), |mut acc: Vec<(u32, u32)>, line| {
            line.clone().enumerate().for_each(|(i, c)| match c {
                '0' => {
                    if let Some(elem) = acc.get_mut(i) {
                        *elem = (elem.0 + 1, elem.1);
                    } else {
                        acc.insert(i, (1, 0));
                    }
                }
                '1' => {
                    if let Some(elem) = acc.get_mut(i) {
                        *elem = (elem.0, elem.1 + 1);
                    } else {
                        acc.insert(i, (0, 1));
                    }
                }
                _ => unreachable!(),
            });

            acc
        })
        .iter()
        .fold((Vec::new(), Vec::new()), |(mut gamma, mut eps), tuple| {
            if tuple.0 > tuple.1 {
                gamma.push('0');
                eps.push('1');
            } else {
                gamma.push('1');
                eps.push('0');
            }

            (gamma, eps)
        });

    let gamma_rate = u32::from_str_radix(&(most_sig_bits.iter().collect::<String>()), 2).unwrap();
    let epsilon_rate =
        u32::from_str_radix(&(least_sig_bits.iter().collect::<String>()), 2).unwrap();

    println!("gamma: {}, epsilon: {}", gamma_rate, epsilon_rate);
    println!("Answer: {}", gamma_rate * epsilon_rate);
}
