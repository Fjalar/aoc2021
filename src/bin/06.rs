use itertools::Itertools;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let mut fishes = input
        .strip_suffix("\n")
        .unwrap()
        .split(',')
        //.inspect(|t| println!("{t}"))
        .map(|substr| substr.parse::<u8>().unwrap())
        .collect_vec();

    (0..80).for_each(|_| {
        let mut fishes_to_add = 0;
        for fish in &mut fishes {
            if let Some(res) = fish.checked_sub(1) {
                *fish = res;
            } else {
                *fish = 6;
                fishes_to_add += 1;
            }
        }
        (0..fishes_to_add).for_each(|_| {
            fishes.push(8u8);
        })
    });

    Some(fishes.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut fishes = input
        .strip_suffix("\n")
        .unwrap()
        .split(',')
        .map(|substr| substr.parse::<u8>().unwrap())
        .collect_vec();

    (0..256).for_each(|_| {
        let mut fishes_to_add = 0;
        for fish in &mut fishes {
            if let Some(res) = fish.checked_sub(1) {
                *fish = res;
            } else {
                *fish = 6;
                fishes_to_add += 1;
            }
        }
        (0..fishes_to_add).for_each(|_| {
            fishes.push(8u8);
        })
    });

    Some(fishes.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5934));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(26_984_457_539));
    }
}
