advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let mut octo = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>();

    let bounds = (octo.len() - 1, octo[0].len() - 1);

    let total_flashes = (0..100).map(|_| increment(&mut octo, bounds)).sum::<u64>();

    Some(total_flashes)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn print_energy(octo: &Vec<Vec<u8>>) {
    for row in octo {
        for col in row {
            print!("{col}");
        }
        println!();
    }
}

fn increment(octo: &mut [Vec<u8>], bounds: (usize, usize)) -> u64 {
    // First increment
    octo.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|energy| {
            *energy = energy.saturating_add(1);
        })
    });

    let mut flashes = octo
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter().enumerate().filter_map(move |(col_idx, col)| {
                if col > &9 {
                    Some((row_idx, col_idx))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();

    if flashes.is_empty() {
        return 0;
    }

    let mut idx_of_next_to_check = 0usize;

    loop {
        let (r, c) = flashes[idx_of_next_to_check];
        let row_range = r.saturating_sub(1)..=(bounds.0.min(r + 1));
        let col_range = c.saturating_sub(1)..=(bounds.1.min(c + 1));

        row_range.for_each(|row| {
            col_range.clone().for_each(|col| {
                if !flashes.contains(&(row, col)) {
                    octo[row][col] = octo[row][col].saturating_add(1);
                    if octo[row][col] > 9 {
                        flashes.push((row, col));
                    }
                }
            })
        });

        idx_of_next_to_check += 1;

        if idx_of_next_to_check >= flashes.len() {
            break;
        }
    }

    // Reset flashed cells to zero
    octo.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|energy| {
            if *energy > 9 {
                *energy = 0;
            }
        })
    });

    // println!("{flashes:?}");
    // println!("{} flashes", flashes.len());

    flashes.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1656));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
