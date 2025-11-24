use std::{collections::VecDeque, rc::Rc};

use itertools::Itertools;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u64> {
    let total_score = input
        .lines()
        .map(|line| {
            let mut open_brackets = VecDeque::new();
            for c in line.chars() {
                if matches!(c, '(' | '[' | '{' | '<') {
                    open_brackets.push_back(c);
                } else if let Some(latest_open) = open_brackets.pop_back() {
                    if corresponding_closing(latest_open) != c {
                        return part_1_score(c);
                    }
                }
            }
            0
        })
        .sum::<u64>();

    Some(total_score)
}

pub fn part_two(input: &str) -> Option<u64> {
    let total_scores = input
        .lines()
        .map(|line| {
            let mut open_brackets = VecDeque::new();
            for c in line.chars() {
                if matches!(c, '(' | '[' | '{' | '<') {
                    open_brackets.push_back(c);
                } else if let Some(latest_open) = open_brackets.pop_back() {
                    if corresponding_closing(latest_open) != c {
                        return 0;
                    }
                }
            }
            open_brackets
                .iter()
                .rev()
                .fold(0, |acc, &c| acc * 5 + part_2_score(c))
        })
        .filter(|&score| score > 0)
        .sorted()
        .collect::<Rc<[u64]>>();

    Some(total_scores[total_scores.len() / 2])
}

const fn corresponding_closing(bracket: char) -> char {
    match bracket {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!(),
    }
}

const fn part_1_score(bracket: char) -> u64 {
    match bracket {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!(),
    }
}

const fn part_2_score(bracket: char) -> u64 {
    match bracket {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(26397));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288957));
    }
}
