use std::rc::Rc;

use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Rc<[u8]>>()
        })
        .collect::<Rc<[_]>>();

    let upper_bound_i = map.len();
    let upper_bound_j = map[0].len();

    let mut ans = 0;

    for i in 0..upper_bound_i {
        for j in 0..upper_bound_j {
            let center = map[i][j];

            if i > 0 && map[i - 1][j] <= center {
                continue;
            }

            if j > 0 && map[i][j - 1] <= center {
                continue;
            }

            if i < upper_bound_i - 1 && map[i + 1][j] <= center {
                continue;
            }

            if j < upper_bound_j - 1 && map[i][j + 1] <= center {
                continue;
            }

            // println!("Low point at {i},{j}");
            ans += (1 + center) as u64;
        }
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Rc<[u8]>>()
        })
        .collect::<Rc<[_]>>();

    let upper_bound_i = map.len();
    let upper_bound_j = map[0].len();

    let mut low_points = Vec::new();

    for i in 0..upper_bound_i {
        for j in 0..upper_bound_j {
            let center = map[i][j];

            if (i > 0 && map[i - 1][j] <= center)
                || (j > 0 && map[i][j - 1] <= center)
                || (i < upper_bound_i - 1 && map[i + 1][j] <= center)
                || (j < upper_bound_j - 1 && map[i][j + 1] <= center)
            {
                continue;
            } else {
                low_points.push((i, j));
            }
        }
    }

    let basin_sizes = low_points
        .iter()
        .map(|(i, j)| {
            let mut current_basin = vec![(*i, *j)];

            let mut first_unchecked: usize = 0;

            // println!("Checking {i},{j}");

            loop {
                let (i, j) = current_basin[first_unchecked];
                // println!("\tchecking {i},{j}");

                if check_if_in_basin(i as isize - 1, j as isize, &map)
                    && !current_basin.contains(&((i - 1), j))
                {
                    // println!("Found that it contains {},{j}", i - 1);
                    current_basin.push(((i - 1), j));
                }
                if check_if_in_basin(i as isize + 1, j as isize, &map)
                    && !current_basin.contains(&((i + 1), j))
                {
                    // println!("Found that it contains {},{j}", i + 1);
                    current_basin.push(((i + 1), j));
                }
                if check_if_in_basin(i as isize, j as isize - 1, &map)
                    && !current_basin.contains(&(i, (j - 1)))
                {
                    // println!("Found that it contains {i},{}", j - 1);
                    current_basin.push((i, (j - 1)));
                }
                if check_if_in_basin(i as isize, j as isize + 1, &map)
                    && !current_basin.contains(&(i, (j + 1)))
                {
                    // println!("Found that it contains {i},{}", j + 1);
                    current_basin.push((i, (j + 1)));
                }

                first_unchecked += 1;

                if first_unchecked >= current_basin.len() {
                    break;
                }
            }
            current_basin.len() as u64
        })
        .sorted()
        .rev()
        .take(3)
        .product();

    Some(basin_sizes)
}

fn check_if_in_basin(i: isize, j: isize, map: &Rc<[Rc<[u8]>]>) -> bool {
    let upper_bound_i = map.len() as isize;
    let upper_bound_j = map[0].len() as isize;

    if (0..upper_bound_i).contains(&i) && (0..upper_bound_j).contains(&j) {
        return map[i as usize][j as usize] != 9;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(15));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1134));
    }
}
