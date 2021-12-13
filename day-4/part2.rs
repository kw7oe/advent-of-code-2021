use std::collections::HashMap;

fn main() {
    let (draw_numbers, boards, _) = input();

    println!("{:?}", draw_numbers);
    print_boards(&boards);

    let (last_number, score) = process(&draw_numbers, &boards);
    println!("{} x {} = {}", last_number, score, last_number * score);
}

fn input() -> (Vec<u8>, Vec<Vec<Vec<u8>>>, usize) {
    include_str!("input.txt").lines().fold(
        (Vec::new(), Vec::new(), 0),
        |(mut draw_numbers, mut boards, board_index), s| {
            if s.is_empty() {
                let index_increment = if boards.is_empty() { 0 } else { 1 };
                return (draw_numbers, boards, board_index + index_increment);
            }

            if draw_numbers.is_empty() && board_index == 0 {
                draw_numbers = s
                    .split(",")
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<u8>>();
            } else {
                let row: Vec<u8> = s
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<u8>>();

                match boards.get_mut(board_index) {
                    None => {
                        boards.insert(board_index, vec![row]);
                    }
                    Some(board) => {
                        board.push(row);
                    }
                }
            }

            (draw_numbers, boards, board_index)
        },
    )
}

fn process(draw_numbers: &Vec<u8>, boards: &Vec<Vec<Vec<u8>>>) -> (u32, u32) {
    let mut win_number: Option<u8> = None;
    let mut win_board: Option<usize> = None;

    let mut rows_info: Vec<HashMap<usize, u8>> = vec![HashMap::new(); boards.len()];
    let mut cols_info: Vec<HashMap<usize, u8>> = vec![HashMap::new(); boards.len()];

    for number in draw_numbers {
        for (index, board) in boards.iter().enumerate() {
            if let Some((row_index, col_index)) = mark(board, *number) {
                let row_info = rows_info.get_mut(index).unwrap();
                let col_info = cols_info.get_mut(index).unwrap();

                let win_row = row_info.iter().find(|(_, &x)| x == 5);
                let win_col = col_info.iter().find(|(_, &x)| x == 5);

                if win_row.is_none() && win_col.is_none() {
                    let row_count = row_info.entry(row_index).or_insert(0);
                    *row_count += 1;

                    let col_count = col_info.entry(col_index).or_insert(0);
                    *col_count += 1;

                    if *row_count == 5 || *col_count == 5 {
                        win_number = Some(*number);
                        win_board = Some(index);
                    }
                }
            }
        }
    }

    let used_num = draw_numbers
        .split_inclusive(|&x| x == win_number.unwrap())
        .next()
        .unwrap();

    let unmarked: Vec<u32> = boards
        .get(win_board.unwrap())
        .unwrap()
        .iter()
        .flatten()
        .filter(|&n| !(used_num.iter().any(|u| u == n)))
        .map(|&n| n as u32)
        .collect();

    (win_number.unwrap().into(), unmarked.iter().sum())
}

fn mark(board: &Vec<Vec<u8>>, number: u8) -> Option<(usize, usize)> {
    board
        .iter()
        .enumerate()
        .fold(None, |acc, (row_index, row)| {
            match row.iter().position(|&n| n == number) {
                None => acc,
                Some(col_index) => Some((row_index, col_index)),
            }
        })
}

fn print_boards(boards: &Vec<Vec<Vec<u8>>>) {
    for board in boards {
        for row in board {
            println!("{:?}", row);
        }

        println!("");
    }
}
