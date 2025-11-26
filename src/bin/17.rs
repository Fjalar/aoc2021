use itertools::Itertools;

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<u64> {
    let (left, right) = input
        .trim()
        .strip_prefix("target area: x=")
        .unwrap()
        .split_once(", y=")
        .unwrap();

    let (x_low, x_high) = left.split_once("..").unwrap();
    let (y_low, y_high) = right.split_once("..").unwrap();

    let bounds = Bounds {
        x_low: x_low.parse::<isize>().unwrap(),
        x_high: x_high.parse::<isize>().unwrap(),
        y_low: y_low.parse::<isize>().unwrap(),
        y_high: y_high.parse::<isize>().unwrap(),
    };

    // Somewhat arbitrary bounds
    let max_y = (0..bounds.x_high)
        .cartesian_product(bounds.y_low..1000)
        .filter_map(|(x, y)| test_trajectory((x, y), &bounds))
        .max()
        .unwrap();

    // println!("{max_y}");

    Some(max_y as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (left, right) = input
        .trim()
        .strip_prefix("target area: x=")
        .unwrap()
        .split_once(", y=")
        .unwrap();

    let (x_low, x_high) = left.split_once("..").unwrap();
    let (y_low, y_high) = right.split_once("..").unwrap();

    let bounds = Bounds {
        x_low: x_low.parse::<isize>().unwrap(),
        x_high: x_high.parse::<isize>().unwrap(),
        y_low: y_low.parse::<isize>().unwrap(),
        y_high: y_high.parse::<isize>().unwrap(),
    };

    // Somewhat arbitrary bounds
    let count = (0..=bounds.x_high)
        .cartesian_product(bounds.y_low..1000)
        .filter_map(|(x, y)| test_trajectory((x, y), &bounds))
        .count();

    // println!("{max_y}");

    Some(count as u64)
}

struct Bounds {
    pub x_low: isize,
    pub x_high: isize,
    pub y_low: isize,
    pub y_high: isize,
}

fn test_trajectory(velocity: (isize, isize), bounds: &Bounds) -> Option<isize> {
    let (mut x, mut y) = (0isize, 0isize);
    let (mut x_vel, mut y_vel) = velocity;

    let mut highest_y = isize::MIN;

    loop {
        if (bounds.x_low..=bounds.x_high).contains(&x)
            && (bounds.y_low..=bounds.y_high).contains(&y)
        {
            return Some(highest_y);
        }

        x += x_vel;
        x_vel -= x_vel.signum();
        y += y_vel;
        y_vel -= 1;

        highest_y = highest_y.max(y);

        if x > bounds.x_high || y < bounds.y_low {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(112));
    }
}
