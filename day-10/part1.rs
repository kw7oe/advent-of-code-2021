fn main() {
    let mut unmatch = Vec::new();
    include_str!("input.txt").lines().for_each(|line| {
        let mut vec = Vec::new();
        for c in line.chars() {
            if "{([<".contains(c) {
                vec.push(c)
            } else {
                let begin = vec.pop().unwrap();

                if !is_matching(begin, c) {
                    println!("First unmatch: {}", c);
                    unmatch.push(c);
                    break;
                }
            }
        }
    });

    let mut total = 0;
    for um in unmatch {
        total += get_score(um);
    }

    println!("Total: {}", total);
}

fn get_score(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn is_matching(begin: char, end: char) -> bool {
    let expected_end = match begin {
        '{' => '}',
        '(' => ')',
        '[' => ']',
        '<' => '>',
        _ => unreachable!(),
    };

    expected_end == end
}
