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
    let mut mature = [0u64; 7];
    let mut babies = [0u64; 2];
    input
        .strip_suffix("\n")
        .unwrap()
        .split(',')
        .map(|substr| substr.parse::<u8>().unwrap())
        .for_each(|initial_time| mature[initial_time as usize] += 1);

    let mut mature_idx = 0usize;
    let mut baby_idx = 0;
    (0..256).for_each(|_| {
        let to_birth = mature[mature_idx];
        // println!("at time {t}, {to_birth} births");
        let new_baby_idx = baby_idx ^ 1;
        mature[mature_idx] += babies[baby_idx];
        babies[baby_idx] = to_birth;
        baby_idx = new_baby_idx;

        mature_idx = (mature_idx + 1) % 7;
    });

    Some(mature.iter().sum::<u64>() + babies.iter().sum::<u64>())
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
