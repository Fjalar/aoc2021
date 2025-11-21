use itertools::Itertools;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let first_line = lines.next();
    let draws = first_line
        .unwrap()
        .split(',')
        .map(|num| num.parse::<u64>().unwrap())
        .collect_vec();

    let mut parsed_lines = lines.filter(|line| !line.is_empty()).map(|line| {
        line.split_whitespace()
            .map(|num| Some(num.parse::<u64>().unwrap()))
            .collect_array::<5>()
            .unwrap()
    });

    let mut boards = Vec::<[[Option<u64>; 5]; 5]>::new();

    while let Some(bla) = parsed_lines.next_array::<5>() {
        boards.push(bla);
    }

    for num in draws {
        {
            for board in boards.iter_mut() {
                for line in board {
                    line.iter_mut().for_each(|board_option| {
                        if let Some(board_num) = board_option {
                            if *board_num == num {
                                *board_option = None;
                            }
                        }
                    });
                }
            }
        }
        for board in &boards {
            if let Some(score) = check_win(board) {
                return Some(score * num);
            }
        }
    }

    None
}

fn check_win(board: &[[Option<u64>; 5]; 5]) -> Option<u64> {
    if board
        .iter()
        .any(|line| line.iter().all(|board_option| board_option.is_none()))
        || (0..5).any(|col| board.iter().all(|line| line[col].is_none()))
    //|| (0..5).all(|diag| board[diag][diag].is_none())
    //|| (0..5).all(|diag| board[diag][4 - diag].is_none())
    {
        Some(
            board
                .as_flattened()
                .iter()
                .filter_map(|opt| *opt)
                .sum::<u64>(),
        )
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let first_line = lines.next();
    let draws = first_line
        .unwrap()
        .split(',')
        .map(|num| num.parse::<u64>().unwrap())
        .collect_vec();

    let mut parsed_lines = lines.filter(|line| !line.is_empty()).map(|line| {
        line.split_whitespace()
            .map(|num| Some(num.parse::<u64>().unwrap()))
            .collect_array::<5>()
            .unwrap()
    });

    let mut boards = Vec::<[[Option<u64>; 5]; 5]>::new();

    while let Some(bla) = parsed_lines.next_array::<5>() {
        boards.push(bla);
    }

    for num in draws {
        {
            for board in boards.iter_mut() {
                for line in board {
                    line.iter_mut().for_each(|board_option| {
                        if let Some(board_num) = board_option {
                            if *board_num == num {
                                *board_option = None;
                            }
                        }
                    });
                }
            }
        }

        if boards.len() == 1 {
            if let Some(score) = check_win(&boards[0]) {
                return Some(score * num);
            }
        }
        boards.retain(|board| check_win(board).is_none())
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4512));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1924));
    }
}
