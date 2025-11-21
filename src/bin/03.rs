use itertools::Itertools;
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let num_length = input.find("\n").unwrap();

    let mut freqs = vec![0i32; num_length];

    input.lines().for_each(|line| {
        line.chars()
            .enumerate()
            .for_each(|(idx, c)| match c.to_digit(2).unwrap() {
                0 => freqs[idx] -= 1,
                1 => freqs[idx] += 1,
                _ => panic!(),
            })
    });

    let gamma = freqs
        .iter()
        .map(|&val| if val > 0 { 1 } else { 0 })
        .fold(0u64, |acc, x| (acc << 1) + x);

    let epsilon = freqs
        .iter()
        .map(|&val| if val > 0 { 0 } else { 1 })
        .fold(0u64, |acc, x| (acc << 1) + x);

    Some(gamma * epsilon)
}

pub fn part_two(input: &str) -> Option<u64> {
    let nums = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(2).unwrap() as u64)
                .collect_vec()
        })
        .collect_vec();

    let mut oxygen = nums.clone();
    let mut idx = 0;
    while oxygen.len() > 1 {
        let freq: i64 = oxygen
            .iter()
            .map(|v| if v[idx] == 1 { 1 } else { -1 })
            .sum();
        if freq < 0 {
            oxygen.retain(|v| v[idx] == 0);
        } else {
            oxygen.retain(|v| v[idx] == 1);
        }
        idx += 1;
    }

    let mut co2 = nums.clone();
    idx = 0;
    while co2.len() > 1 {
        let freq: i64 = co2.iter().map(|v| if v[idx] == 1 { 1 } else { -1 }).sum();
        if freq >= 0 {
            co2.retain(|v| v[idx] == 0);
        } else {
            co2.retain(|v| v[idx] == 1);
        }
        idx += 1;
    }

    println!("{:?}", &oxygen[0]);
    println!("{:?}", &oxygen[0]);

    let oxygen_rating = oxygen[0].iter().fold(0u64, |acc, x| (acc << 1) + x);
    let co2_rating = co2[0].iter().fold(0u64, |acc, x| (acc << 1) + x);

    Some(oxygen_rating * co2_rating)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(198));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(230));
    }
}
