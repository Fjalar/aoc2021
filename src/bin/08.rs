use itertools::Itertools;

advent_of_code::solution!(8);

// Removed every other newline in the example input to actually match the real input

pub fn part_one(input: &str) -> Option<u64> {
    let digit_count = input
        .lines()
        .flat_map(|line| {
            let (_, right) = line.split_once('|').unwrap();
            right.split_whitespace()
        })
        .filter(|s| matches!(s.len(), 2 | 3 | 4 | 7))
        .count();

    Some(digit_count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let sum = input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once('|').unwrap();
            let digit_maps = left
                .split_whitespace()
                .map(str_to_u8)
                .collect_array::<10>()
                .unwrap();

            // for m in digit_maps {
            //     println!("{m:07b}");
            // }

            // 0..=9]
            let mut correct_patterns = [0u8; 10];

            // one
            correct_patterns[1] = *digit_maps
                .iter()
                .filter(|u| u.count_ones() == 2)
                .exactly_one()
                .unwrap();
            // println!("Matched {:07b} with 1", correct_patterns[1]);

            // four
            correct_patterns[4] = *digit_maps
                .iter()
                .filter(|u| u.count_ones() == 4)
                .exactly_one()
                .unwrap();
            // println!("Matched {:07b} with 4", correct_patterns[4]);

            // seven
            correct_patterns[7] = *digit_maps
                .iter()
                .filter(|u| u.count_ones() == 3)
                .exactly_one()
                .unwrap();
            // println!("Matched {:07b} with 7", correct_patterns[7]);

            // eight
            correct_patterns[8] = *digit_maps
                .iter()
                .filter(|u| u.count_ones() == 7)
                .exactly_one()
                .unwrap();
            // println!("Matched {:07b} with 8", correct_patterns[8]);

            // six
            correct_patterns[6] = *digit_maps
                .iter()
                .filter(|&u| u.count_ones() == 6 && (*u & correct_patterns[1]).count_ones() == 1)
                .exactly_one()
                .unwrap();
            // println!("Matched {:07b} with 6", correct_patterns[6]);

            // five
            correct_patterns[5] = *digit_maps
                .iter()
                .filter(|&u| u.count_ones() == 5 && (*u ^ correct_patterns[6]).count_ones() == 1)
                .exactly_one()
                .unwrap();
            // println!("Matched {:07b} with 5", correct_patterns[5]);

            // nine
            correct_patterns[9] = *digit_maps
                .iter()
                .filter(|&u| {
                    u.count_ones() == 6
                        && *u != correct_patterns[6]
                        && (*u ^ correct_patterns[4]).count_ones() == 2
                })
                .exactly_one()
                .unwrap();
            // println!("Matched {:07b} with 9", correct_patterns[9]);

            // zero
            correct_patterns[0] = *digit_maps
                .iter()
                .filter(|&u| {
                    u.count_ones() == 6
                        && (*u != correct_patterns[6])
                        && (*u != correct_patterns[9])
                })
                .exactly_one()
                .unwrap();
            // println!("Matched {:07b} with 0", correct_patterns[0]);

            // two
            correct_patterns[2] = *digit_maps
                .iter()
                .filter(|&u| {
                    u.count_ones() == 5
                        && (*u != correct_patterns[5])
                        && (*u ^ correct_patterns[1]).count_ones() == 5
                })
                .exactly_one()
                .unwrap();
            // println!("Matched {:07b} with 2", correct_patterns[2]);

            // three
            correct_patterns[3] = *digit_maps
                .iter()
                .filter(|&u| {
                    u.count_ones() == 5
                        && (*u != correct_patterns[2])
                        && (*u != correct_patterns[5])
                })
                .exactly_one()
                .unwrap();
            // println!("Matched {:07b} with 3", correct_patterns[3]);

            // println!("---");
            // for pattern in correct_patterns {
            //     println!("{pattern:07b}");
            // }

            right
                .split_whitespace()
                .enumerate()
                .map(|(idx, s)| (idx, str_to_u8(s)))
                .map(move |(idx, u)| {
                    (10usize.pow(3 - idx as u32)
                        * correct_patterns
                            .iter()
                            .position(|&pattern| pattern == u)
                            .unwrap()) as u64
                })
                .sum::<u64>()
        })
        .sum::<u64>();

    Some(sum)
}

fn str_to_u8(s: &str) -> u8 {
    s.chars().fold(0, |acc, c| acc + (1 << ((c as u8) - 97)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(26));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(61229));
    }
}
