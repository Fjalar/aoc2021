use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u64> {
    let (template, rules) = input.split_once("\n\n").unwrap();
    let template = template
        .chars()
        .tuple_windows::<(char, char)>()
        .collect_vec();

    let rules = rules
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(" -> ").unwrap();
            let left = left.chars().collect_tuple::<(char, char)>().unwrap();
            let right = right.chars().exactly_one().unwrap();

            // NC -> H
            // (('N', 'C'), (('N', 'H'), ('H', 'C')))

            (left, ((left.0, right), (right, left.1)))
        })
        .collect_vec();

    let mut counts = vec![0u64; rules.len()];

    let mapping = (0..rules.len())
        .map(|i| {
            // Finds the position in the list of the two resulting bigrams from an insertion
            let left_mapping = rules
                .iter()
                .position(|(rule_key, _)| rules[i].1 .0 == *rule_key)
                .unwrap();
            let right_mapping = rules
                .iter()
                .position(|(rule_key, _)| rules[i].1 .1 == *rule_key)
                .unwrap();

            (left_mapping, right_mapping)
        })
        .collect_vec();

    template.iter().for_each(|bigram| {
        let pos = rules
            .iter()
            .position(|(rule_key, _)| bigram == rule_key)
            .unwrap();
        counts[pos] += 1;
    });

    for _ in 0..10 {
        let mut new_counts = vec![0u64; counts.len()];

        for i in 0..counts.len() {
            let bigram_count = counts[i];
            let (left_mapping, right_mapping) = mapping[i];
            new_counts[left_mapping] += bigram_count;
            new_counts[right_mapping] += bigram_count;
        }

        counts = new_counts;
    }

    let mut unigram_set = rules
        .iter()
        .flat_map(|(key, _)| [(key.0, 0u64), (key.1, 0u64)])
        .unique()
        .collect::<HashMap<char, u64>>();

    // println!("{unigram_set:?}");

    for i in 0..counts.len() {
        let (left_letter, right_letter) = rules[i].0;
        if let Some(left_count) = unigram_set.get_mut(&left_letter) {
            *left_count += counts[i];
        } else {
            panic!()
        }
        if let Some(right_count) = unigram_set.get_mut(&right_letter) {
            *right_count += counts[i];
        } else {
            panic!()
        }
    }

    if let Some(left_count) = unigram_set.get_mut(&template.first().unwrap().0) {
        *left_count += 1;
    } else {
        panic!()
    }
    if let Some(right_count) = unigram_set.get_mut(&template.last().unwrap().1) {
        *right_count += 1;
    } else {
        panic!()
    }

    let max_count = unigram_set.iter().max_by_key(|(_, val)| **val).unwrap().1 / 2;
    let min_count = unigram_set.iter().min_by_key(|(_, val)| **val).unwrap().1 / 2;

    // println!("{unigram_set:?}");

    // println!("{max_count}");
    // println!("{min_count}");

    Some(max_count - min_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (template, rules) = input.split_once("\n\n").unwrap();
    let template = template
        .chars()
        .tuple_windows::<(char, char)>()
        .collect_vec();

    let rules = rules
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(" -> ").unwrap();
            let left = left.chars().collect_tuple::<(char, char)>().unwrap();
            let right = right.chars().exactly_one().unwrap();

            // NC -> H
            // (('N', 'C'), (('N', 'H'), ('H', 'C')))

            (left, ((left.0, right), (right, left.1)))
        })
        .collect_vec();

    let mut counts = vec![0u64; rules.len()];

    let mapping = (0..rules.len())
        .map(|i| {
            // Finds the position in the list of the two resulting bigrams from an insertion
            let left_mapping = rules
                .iter()
                .position(|(rule_key, _)| rules[i].1 .0 == *rule_key)
                .unwrap();
            let right_mapping = rules
                .iter()
                .position(|(rule_key, _)| rules[i].1 .1 == *rule_key)
                .unwrap();

            (left_mapping, right_mapping)
        })
        .collect_vec();

    template.iter().for_each(|bigram| {
        let pos = rules
            .iter()
            .position(|(rule_key, _)| bigram == rule_key)
            .unwrap();
        counts[pos] += 1;
    });

    for _ in 0..40 {
        let mut new_counts = vec![0u64; counts.len()];

        for i in 0..counts.len() {
            let bigram_count = counts[i];
            let (left_mapping, right_mapping) = mapping[i];
            new_counts[left_mapping] += bigram_count;
            new_counts[right_mapping] += bigram_count;
        }

        counts = new_counts;
    }

    let mut unigram_set = rules
        .iter()
        .flat_map(|(key, _)| [(key.0, 0u64), (key.1, 0u64)])
        .unique()
        .collect::<HashMap<char, u64>>();

    // println!("{unigram_set:?}");

    for i in 0..counts.len() {
        let (left_letter, right_letter) = rules[i].0;
        if let Some(left_count) = unigram_set.get_mut(&left_letter) {
            *left_count += counts[i];
        } else {
            panic!()
        }
        if let Some(right_count) = unigram_set.get_mut(&right_letter) {
            *right_count += counts[i];
        } else {
            panic!()
        }
    }

    if let Some(left_count) = unigram_set.get_mut(&template.first().unwrap().0) {
        *left_count += 1;
    } else {
        panic!()
    }
    if let Some(right_count) = unigram_set.get_mut(&template.last().unwrap().1) {
        *right_count += 1;
    } else {
        panic!()
    }

    let max_count = unigram_set.iter().max_by_key(|(_, val)| **val).unwrap().1 / 2;
    let min_count = unigram_set.iter().min_by_key(|(_, val)| **val).unwrap().1 / 2;

    // println!("{unigram_set:?}");

    // println!("{max_count}");
    // println!("{min_count}");

    Some(max_count - min_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1588));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2_188_189_693_529));
    }
}
