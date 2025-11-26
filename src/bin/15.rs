use std::iter;

use itertools::Itertools;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u64> {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Box<[u8]>>()
        })
        .collect::<Box<_>>();

    let end = (grid[0].len() - 1, grid.len() - 1);

    let mut unvisited = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, _)| (x, y)))
        .rev()
        .collect_vec();

    let mut shortest = grid
        .iter()
        .map(|row| row.iter().map(|_| u32::MAX).collect::<Box<[u32]>>())
        .collect::<Box<_>>();

    shortest[0][0] = 0;

    // while there is a reachable node left unvisited
    while !unvisited.is_empty() && !unvisited.iter().all(|&(x, y)| shortest[x][y] == u32::MAX) {
        // sort unvisited so that last element has the shortest path
        unvisited.sort_by_key(|&(x, y)| u32::MAX - shortest[x][y]);
        let current_shortest = unvisited.pop().unwrap();

        let distance_here = shortest[current_shortest.0][current_shortest.1];

        for delta in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            if let (Some(x), Some(y)) = (
                current_shortest.0.checked_add_signed(delta.0),
                current_shortest.1.checked_add_signed(delta.1),
            ) {
                if x <= end.0 && y <= end.1 {
                    let new_distance = distance_here + grid[x][y] as u32;
                    if new_distance < shortest[x][y] {
                        shortest[x][y] = new_distance;
                    }
                }
            }
        }
    }

    // for row in &shortest {
    //     println!("{row:?}");
    // }

    Some(shortest[end.0][end.1] as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let wide = input.lines().map(|line| {
        let risks = line.chars().map(|c| c.to_digit(10).unwrap() as u8);

        iter::repeat_n(risks, 5)
            .enumerate()
            .flat_map(|(cycle, inner_iter)| {
                inner_iter.map(move |risk| {
                    if (risk + cycle as u8) > 9 {
                        risk + cycle as u8 - 9
                    } else {
                        risk + cycle as u8
                    }
                })
            })
    });

    let grid = iter::repeat_n(wide, 5)
        .enumerate()
        .flat_map(|(cycle, tall_row)| {
            tall_row.map(move |row| {
                row.map(move |risk| {
                    if (risk + cycle as u8) > 9 {
                        risk + cycle as u8 - 9
                    } else {
                        risk + cycle as u8
                    }
                })
                .collect::<Box<[u8]>>()
            })
        })
        .collect::<Box<_>>();

    let end = (grid[0].len() - 1, grid.len() - 1);

    // for row in &grid {
    //     println!("{row:?}");
    // }

    let mut unvisited = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, _)| (x, y)))
        .rev()
        .collect_vec();

    let mut shortest = grid
        .iter()
        .map(|row| row.iter().map(|_| u32::MAX).collect::<Box<[u32]>>())
        .collect::<Box<_>>();

    shortest[0][0] = 0;

    // while there is a reachable node left unvisited
    while !unvisited.is_empty() && !unvisited.iter().all(|&(x, y)| shortest[x][y] == u32::MAX) {
        // sort unvisited so that last element has the shortest path
        unvisited.sort_by_key(|&(x, y)| u32::MAX - shortest[x][y]);
        let current_shortest = unvisited.pop().unwrap();

        println!("Visiting {current_shortest:?}");

        let distance_here = shortest[current_shortest.0][current_shortest.1];

        for delta in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            if let (Some(x), Some(y)) = (
                current_shortest.0.checked_add_signed(delta.0),
                current_shortest.1.checked_add_signed(delta.1),
            ) {
                if x <= end.0 && y <= end.1 {
                    // SAFETY: we have already performed the bounds checks using checked_add_sign and x, y <= end
                    unsafe {
                        let new_distance =
                            distance_here + *grid.get_unchecked(x).get_unchecked(y) as u32;
                        if new_distance < *shortest.get_unchecked(x).get_unchecked(y) {
                            *shortest.get_unchecked_mut(x).get_unchecked_mut(y) = new_distance;
                        }
                    }
                }
            }
        }
    }

    // for row in &shortest {
    //     println!("{row:?}");
    // }

    Some(shortest[end.0][end.1] as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(315));
    }
}
