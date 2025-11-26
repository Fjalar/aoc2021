use itertools::Itertools;

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<u64> {
    let bits = input
        .trim()
        .chars()
        .flat_map(|c| {
            let byte = c.to_digit(16).unwrap() as u8;
            (0..4).rev().map(move |i| byte >> i & 1)
        })
        .collect_vec();

    // for bit in &bits {
    //     print!("{bit}");
    // }
    // println!();

    let mut pointer = 0usize;

    let answer = parse_packet(&bits, &mut pointer);

    // println!("{answer}");

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u64> {
    let bits = input
        .trim()
        .chars()
        .flat_map(|c| {
            let byte = c.to_digit(16).unwrap() as u8;
            (0..4).rev().map(move |i| byte >> i & 1)
        })
        .collect_vec();

    // for bit in &bits {
    //     print!("{bit}");
    // }
    // println!();

    let mut pointer = 0usize;

    let answer = parse_packet_part_2(&bits, &mut pointer);

    // println!("{answer}");

    Some(answer)
}

fn bits_to_value(bits: &[u8]) -> u64 {
    bits.iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (idx, x)| acc + (*x as u64 * 2u64.pow(idx as u32)))
}

fn parse_packet(bits: &[u8], pointer: &mut usize) -> u64 {
    let version = bits_to_value(&bits[*pointer..(*pointer + 3)]);
    *pointer += 3;
    let type_id = bits_to_value(&bits[(*pointer)..(*pointer + 3)]);
    *pointer += 3;
    // println!("Version is {version}");
    // println!("Type ID is {type_id}");

    version
        + match type_id {
            4 => {
                parse_literal(bits, pointer);
                0
            }
            _ => {
                let length_type_id = bits[*pointer];

                *pointer += 1;

                if length_type_id == 0 {
                    parse_length(bits, pointer)
                } else {
                    parse_count(bits, pointer)
                }
            }
        }
}

fn parse_packet_part_2(bits: &[u8], pointer: &mut usize) -> u64 {
    // let version = bits_to_value(&bits[*pointer..(*pointer + 3)]);
    *pointer += 3;
    let type_id = bits_to_value(&bits[(*pointer)..(*pointer + 3)]);
    *pointer += 3;
    // println!("Version is {version}");
    // println!("Type ID is {type_id}");

    match type_id {
        4 => parse_literal(bits, pointer),
        _ => {
            let length_type_id = bits[*pointer];

            *pointer += 1;

            // println!("Length type ID: {length_type_id}");
            if length_type_id == 0 {
                parse_length_part_2(bits, pointer, type_id as u8)
            } else {
                parse_count_part_2(bits, pointer, type_id as u8)
            }
        }
    }
}

fn parse_literal(bits: &[u8], pointer: &mut usize) -> u64 {
    let mut local_bits = Vec::new();
    loop {
        let packet = &bits[*pointer..(*pointer + 5)];
        *pointer += 5;
        local_bits.extend_from_slice(&packet[1..]);

        if packet[0] == 0 {
            break;
        }
    }

    let literal = bits_to_value(local_bits.as_slice());

    // println!("Literal value: {}", literal);
    literal
}

fn parse_count(bits: &[u8], pointer: &mut usize) -> u64 {
    let count = bits_to_value(&bits[*pointer..(*pointer + 11)]);
    // println!("Parse count of {count} packets");

    *pointer += 11;

    (0..count).map(|_| parse_packet(bits, pointer)).sum::<u64>()
}

fn parse_length(bits: &[u8], pointer: &mut usize) -> u64 {
    let length = bits_to_value(&bits[*pointer..(*pointer + 15)]);
    // println!("Parse length of {length} bits");

    *pointer += 15;

    let pointer_start = *pointer;

    let mut sum = 0;

    while *pointer < pointer_start + length as usize {
        sum += parse_packet(bits, pointer);
    }

    sum
}

fn operate(mut sub_packets: impl Iterator<Item = u64>, id: u8) -> u64 {
    // println!("Operating with id of {id}");
    match id {
        0 => sub_packets
            // .inspect(|inner| println!("{inner}"))
            .sum(),
        1 => sub_packets.product(),
        2 => sub_packets.min().unwrap(),
        3 => sub_packets.max().unwrap(),
        5 => {
            if sub_packets.next().unwrap() > sub_packets.next().unwrap() {
                1
            } else {
                0
            }
        }
        6 => {
            if sub_packets.next().unwrap() < sub_packets.next().unwrap() {
                1
            } else {
                0
            }
        }
        7 => {
            if sub_packets.next().unwrap() == sub_packets.next().unwrap() {
                1
            } else {
                0
            }
        }
        _ => panic!(),
    }
}

fn parse_count_part_2(bits: &[u8], pointer: &mut usize, id: u8) -> u64 {
    let count = bits_to_value(&bits[*pointer..(*pointer + 11)]);
    // println!("Parse count of {count} packets");

    *pointer += 11;

    let sub_packets = (0..count).map(|_| parse_packet_part_2(bits, pointer));
    // .inspect(|inner| println!("{inner}"));

    operate(sub_packets, id)
}

fn parse_length_part_2(bits: &[u8], pointer: &mut usize, id: u8) -> u64 {
    let length = bits_to_value(&bits[*pointer..(*pointer + 15)]);
    // println!("Parse length of {length} bits");

    *pointer += 15;

    let pointer_start = *pointer;

    let mut sub_packets: Vec<u64> = Vec::new();

    while *pointer < pointer_start + length as usize {
        sub_packets.push(parse_packet_part_2(bits, pointer));
    }

    operate(sub_packets.into_iter(), id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(20));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }
}
