use std::{fmt, ops::Div};

use itertools::Itertools;

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<u64> {
    let result = input
        .lines()
        .map(Brackets::from)
        .reduce(|acc, e| {
            let mut new_snail = acc + e;
            // println!("Before reduction\n{new_snail}");
            new_snail.snail_reduce();
            // println!("Intermediate result {new_snail}");
            new_snail
        })
        .unwrap();

    println!("{result}");

    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[derive(Clone)]
struct Brackets {
    left: Elem,
    right: Elem,
}

#[derive(Clone)]
enum Elem {
    N(Box<Brackets>),
    L(u8),
}

impl fmt::Display for Brackets {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{}]", self.left, self.right)
    }
}

impl std::ops::Add for Brackets {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            left: Elem::N(Box::new(self)),
            right: Elem::N(Box::new(rhs)),
        }
    }
}

impl Brackets {
    fn snail_reduce(&mut self) {
        loop {
            // Explode
            if self.explode(0).is_some() {
                continue;
            }

            // Split
            if self.split() {
                continue;
            }
            break;
        }
    }

    fn explode(&mut self, depth: u8) -> Option<(u8, u8)> {
        // Find first instance of SnailNumber with depth 4, then set that whole number to a zero literal
        // Return the left and right values of the SnailNumber, and add them to the closest adjacent literals

        if depth >= 4 {
            if let (Elem::L(a), Elem::L(b)) = (&self.left, &self.right) {
                // println!("Exploding [{},{}]", *a, *b);
                return Some((*a, *b));
            } else {
                panic!("Number at depth 4 was not just literals: {self}")
            }
        }

        if let Elem::N(brackets) = &mut self.left {
            if let Some(explosion) = brackets.explode(depth + 1) {
                if depth == 3 {
                    self.left = Elem::L(0);
                }
                match &mut self.right {
                    Elem::N(brackets) => brackets.add_to_leftmost(explosion.1),
                    Elem::L(l) => *l += explosion.1,
                }
                return Some((explosion.0, 0));
            }
        }

        if let Elem::N(brackets) = &mut self.right {
            if let Some(explosion) = brackets.explode(depth + 1) {
                if depth == 3 {
                    self.right = Elem::L(0);
                }
                match &mut self.left {
                    Elem::N(brackets) => brackets.add_to_rightmost(explosion.0),
                    Elem::L(l) => *l += explosion.0,
                }
                Some((0, explosion.1))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn add_to_leftmost(&mut self, val: u8) {
        match &mut self.left {
            Elem::N(brackets) => brackets.add_to_leftmost(val),
            Elem::L(l) => {
                *l += val;
            }
        }
    }

    fn add_to_rightmost(&mut self, val: u8) {
        match &mut self.right {
            Elem::N(brackets) => brackets.add_to_rightmost(val),
            Elem::L(l) => {
                *l += val;
            }
        }
    }

    fn split(&mut self) -> bool {
        // Find first instance of literal greater or equal to 10, then replace it with a Snailnumber [(literal/2).floor(), (literal/2).ceil()]

        let mut has_split = false;

        match &mut self.left {
            Elem::N(brackets) => {
                has_split = brackets.split();
            }
            Elem::L(l) => {
                if *l >= 10 {
                    self.left = Elem::N(Box::new(Brackets {
                        left: Elem::L(l.div(2)),
                        right: Elem::L(l.div_ceil(2)),
                    }));
                    return true;
                }
            }
        }

        if !has_split {
            match &mut self.right {
                Elem::N(brackets) => {
                    has_split = brackets.split();
                }
                Elem::L(l) => {
                    if *l >= 10 {
                        self.right = Elem::N(Box::new(Brackets {
                            left: Elem::L(l.div(2)),
                            right: Elem::L(l.div_ceil(2)),
                        }));
                        return true;
                    }
                }
            }
        }

        has_split
    }
}

impl From<&str> for Brackets {
    fn from(value: &str) -> Self {
        let without_brackets = value.strip_prefix("[").unwrap().strip_suffix("]").unwrap();

        let chars = without_brackets.as_bytes();

        let mut bracket_count = 0;
        let mut pos = 0;
        (0..without_brackets.len()).for_each(|i| {
            let current = chars[i] as char;
            if current == '[' {
                bracket_count += 1;
            } else if current == ']' {
                bracket_count -= 1;
            } else if current == ',' && bracket_count == 0 {
                pos = i;
            }
        });

        let (left_str, mut right_str) = without_brackets.split_at(pos);

        if right_str.len() > 1 {
            right_str = &right_str[1..];
        }

        // println!("Split into {left_str}|{right_str}");

        let left = if left_str.len() == 1 {
            Elem::L(left_str.parse::<u8>().unwrap())
        } else {
            Elem::N(Box::new(Self::from(left_str)))
        };
        let right = if right_str.len() == 1 {
            Elem::L(right_str.parse::<u8>().unwrap())
        } else {
            Elem::N(Box::new(Self::from(right_str)))
        };

        Brackets { left, right }
    }
}

impl fmt::Display for Elem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Elem::N(brackets) => write!(f, "{brackets}"),
            Elem::L(l) => write!(f, "{l}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4140));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
