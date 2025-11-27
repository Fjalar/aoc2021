use itertools::Itertools;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u64> {
    let scanners = input
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .filter(|line| !line.starts_with("--- scanner "))
                .map(|line| {
                    line.split(",")
                        .map(|value| value.parse::<i32>().unwrap())
                        .collect_tuple::<(i32, i32, i32)>()
                        .unwrap()
                })
                .collect_vec()
        })
        .collect_vec();

    for scanner in &scanners {
        println!("{scanner:?}");
    }

    println!("{}", scanners.len());

    let diffs = scanners
        .iter()
        .combinations(2)
        .map(|v| {
            v[0].iter()
                .cartesian_product(v[1].iter())
                .map(|(a, b)| a.0 + a.1 + a.2 - b.0 - b.1 - b.2)
                .sum::<i32>()
        })
        .collect_vec();

    println!("{diffs:?}");

    None
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
        assert_eq!(result, Some(79));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
