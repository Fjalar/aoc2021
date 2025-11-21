use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut horizontal: i64 = 0;
    let mut vertical: i64 = 0;
    input
        .lines()
        .map(|line| {
            let mut line_split = line.split_whitespace();
            let left = Direction::from(line_split.next().unwrap());
            let right = line_split.next().unwrap().parse::<i64>().unwrap();
            (left, right)
        })
        .for_each(|(dir, val)| match dir {
            Direction::Up => vertical += val,
            Direction::Down => vertical -= val,
            Direction::Forward => horizontal += val,
            Direction::Back => horizontal -= val,
        });

    Some(vertical.unsigned_abs() * horizontal.unsigned_abs())
}

enum Direction {
    Up,
    Down,
    Forward,
    Back,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "forward" => Direction::Forward,
            "back" => Direction::Back,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => panic!(),
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut horizontal: i64 = 0;
    let mut vertical: i64 = 0;
    let mut aim: i64 = 0;
    input
        .lines()
        .map(|line| {
            let mut line_split = line.split_whitespace();
            let left = Direction::from(line_split.next().unwrap());
            let right = line_split.next().unwrap().parse::<i64>().unwrap();
            (left, right)
        })
        .for_each(|(dir, val)| match dir {
            Direction::Up => aim -= val,
            Direction::Down => {
                aim += val;
            }
            Direction::Forward => {
                horizontal += val;
                vertical += aim * val
            }
            Direction::Back => {
                horizontal -= val;
                vertical -= aim * val
            }
        });

    Some(vertical.unsigned_abs() * horizontal.unsigned_abs())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(150));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(900));
    }
}
