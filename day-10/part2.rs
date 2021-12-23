fn main() {
    let mut scores = Vec::new();
    include_str!("input.txt").lines().for_each(|line| {
        let mut vec = Vec::new();
        let mut incomplete = true;

        for c in line.chars() {
            if "{([<".contains(c) {
                vec.push(c)
            } else {
                let begin = vec.pop().unwrap();

                if !is_matching(begin, c) {
                    incomplete = false;
                    break;
                }
            }
        }

        if incomplete {
            let mut score = 0;
            while let Some(v) = vec.pop() {
                score *= 5;
                score += get_score(v);
            }

            scores.push(score);

            println!("Score: {}", score);
        }
    });

    scores.sort();
    println!("Mid: {}", scores.len() / 2);
    println!("Middle Score: {}", scores[scores.len() / 2]);
}

fn get_score(c: char) -> u64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
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
