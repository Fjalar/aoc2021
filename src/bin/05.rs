use std::collections::HashMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let submarine_lines = input.lines().filter_map(|line| {
        let mut split_line = line.split_whitespace();
        let mut first_pair = split_line.next().unwrap().split(',');
        let pair1 = (
            first_pair.next().unwrap().parse::<u64>().unwrap(),
            first_pair.next().unwrap().parse::<u64>().unwrap(),
        );
        let _ = split_line.next();
        let mut second_pair = split_line.next().unwrap().split(',');
        let pair2 = (
            second_pair.next().unwrap().parse::<u64>().unwrap(),
            second_pair.next().unwrap().parse::<u64>().unwrap(),
        );
        if pair1.0 == pair2.0 || pair1.1 == pair2.1 {
            Some((pair1, pair2))
        } else {
            None
        }
    });

    let mut grid = HashMap::new();

    for line in submarine_lines {
        // println!("{},{} -> {},{}", line.0 .0, line.0 .1, line.1 .0, line.1 .1);
        let x = line.0 .0 as i64;
        let y = line.0 .1 as i64;

        let x_diff = line.1 .0 as i64 - line.0 .0 as i64;
        let y_diff = line.1 .1 as i64 - line.0 .1 as i64;

        if x_diff != 0 {
            let x_range = if x_diff < 0 { x_diff..=0 } else { 0..=x_diff };
            x_range.for_each(|offset| {
                grid.entry((x + offset, y))
                    .and_modify(|n| *n += 1)
                    .or_insert(1);
            });
        } else {
            let y_range = if y_diff < 0 { y_diff..=0 } else { 0..=y_diff };

            y_range.for_each(|offset| {
                grid.entry((x, y + offset))
                    .and_modify(|n| *n += 1)
                    .or_insert(1);
            });
        }
    }

    Some(grid.iter().filter(|(_, val)| *val >= &2).count() as u64)
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
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
