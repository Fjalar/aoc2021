use itertools::Itertools;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u64> {
    let (dots, folds) = input.split_once("\n\n").unwrap();
    let mut dots = dots
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap())
        })
        .collect_vec();

    let folds = folds
        .lines()
        .map(|line| {
            let (axis, value) = line
                .strip_prefix("fold along ")
                .unwrap()
                .split_once('=')
                .unwrap();
            (axis == "x", value.parse::<u32>().unwrap())
        })
        .collect_vec();

    // for fold in folds {
    //     for dot in &mut dots {
    //         if fold.0 && dot.0 > fold.1 {
    //             dot.0 = dot.0 - 2 * (dot.0 - fold.1);
    //         } else if !fold.0 && dot.1 > fold.1 {
    //             dot.1 = dot.1 - 2 * (dot.1 - fold.1);
    //         }
    //     }
    //     dots.sort();
    //     dots.dedup();
    // }

    let fold = folds[0];
    for dot in &mut dots {
        if fold.0 && dot.0 > fold.1 {
            dot.0 = dot.0 - 2 * (dot.0 - fold.1);
        } else if !fold.0 && dot.1 > fold.1 {
            dot.1 = dot.1 - 2 * (dot.1 - fold.1);
        }
    }
    dots.sort();
    dots.dedup();

    // println!("{}", dots.len());
    // println!("{dots:?}");

    // println!("{:?}=={:?}? {}", dots[2], dots[14], dots[2] == dots[14]);

    Some(dots.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (dots, folds) = input.split_once("\n\n").unwrap();
    let mut dots = dots
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap())
        })
        .collect_vec();

    let folds = folds
        .lines()
        .map(|line| {
            let (axis, value) = line
                .strip_prefix("fold along ")
                .unwrap()
                .split_once('=')
                .unwrap();
            (axis == "x", value.parse::<u32>().unwrap())
        })
        .collect_vec();

    for fold in folds {
        for dot in &mut dots {
            if fold.0 && dot.0 > fold.1 {
                dot.0 = dot.0 - 2 * (dot.0 - fold.1);
            } else if !fold.0 && dot.1 > fold.1 {
                dot.1 = dot.1 - 2 * (dot.1 - fold.1);
            }
        }
        dots.sort();
        dots.dedup();
    }

    let mut max_x = 0;
    let mut max_y = 0;

    for dot in &dots {
        max_x = max_x.max(dot.0);
        max_y = max_y.max(dot.1);
    }

    let mut grid = vec![vec![false; (max_x + 1) as usize]; (max_y + 1) as usize];

    for dot in &dots {
        grid[dot.1 as usize][dot.0 as usize] = true;
    }

    // This prints the answer, but would clog up the benchmark if it actually printed
    // for row in grid {
    //     for dot in row {
    //         if dot {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    // Prevent compiler from optimizing everything away because I commented out the output
    // Doesn't work with cargo time anyway for some reason :(
    core::hint::black_box(&mut grid);

    // This puzzle is different, has an alphanumeric answer easiest solved by eye
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(17));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
