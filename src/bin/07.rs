use itertools::Itertools;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let crabs = input
        .strip_suffix('\n')
        .unwrap()
        .split(',')
        .map(|substr| substr.parse::<u32>().unwrap())
        .collect_vec();
    let &max_distance = crabs.iter().max().unwrap();

    let best = (0..=max_distance)
        .map(|pos| crabs.iter().map(|&crab| pos.abs_diff(crab)).sum::<u32>())
        .min()
        .unwrap() as u64;

    Some(best)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
