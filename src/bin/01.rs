advent_of_code::solution!(1);
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| line.parse::<u64>().unwrap())
            .tuple_windows()
            .filter(|(first, second)| second > first)
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| line.parse::<u64>().unwrap())
            .tuple_windows()
            .tuple_windows()
            .filter(|((l1, l2, l3), (r1, r2, r3))| (r1 + r2 + r3) > (l1 + l2 + l3))
            .count() as u64,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }
}
