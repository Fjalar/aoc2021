use std::{fmt, ops::Div};

use itertools::Itertools;

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<u64> {
    // use Elem::L;
    // use Elem::N;
    // let n1 = Brackets {
    //     left: N(Box::new(Brackets {
    //         left: N(Box::new(Brackets {
    //             left: N(Box::new(Brackets {
    //                 left: L(4),
    //                 right: L(3),
    //             })),
    //             right: L(4),
    //         })),
    //         right: L(4),
    //     })),
    //     right: N(Box::new(Brackets {
    //         left: L(7),
    //         right: N(Box::new(Brackets {
    //             left: N(Box::new(Brackets {
    //                 left: L(8),
    //                 right: L(4),
    //             })),
    //             right: L(9),
    //         })),
    //     })),
    // };

    // let n2 = Brackets {
    //     left: L(1),
    //     right: L(1),
    // };

    // let mut n3 = n1 + n2;

    // println!("{}", n3);
    // n3.reduce();
    // println!("{}", n3);

    let vec = input.lines().map(Brackets::from).collect_vec();

    for e in vec {
        println!("{e}");
    }

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
    fn reduce(&mut self) {
        loop {
            // Explode
            let exploded = self.explode(0).is_some();
            println!("Explode? {}\n{}", exploded, self);

            // Split
            let split = self.split();
            println!("Split? {}\n{}", split, self);

            if !exploded && !split {
                break;
            }
        }
    }

    fn explode(&mut self, depth: u8) -> Option<(u8, u8)> {
        // Find first instance of SnailNumber with depth 4, then set that whole number to a zero literal
        // Return the left and right values of the SnailNumber, and add them to the closest adjacent literals

        if depth >= 4 {
            if let (Elem::L(a), Elem::L(b)) = (&self.left, &self.right) {
                return Some((*a, *b));
            } else {
                panic!()
            }
        }

        let mut has_exploded = None;

        match &mut self.left {
            Elem::N(brackets) => {
                if let Some(explosion) = brackets.explode(depth + 1) {
                    has_exploded = Some(explosion);
                    match &mut self.right {
                        Elem::N(bracket) => bracket.add_to_leftmost(explosion.1),
                        Elem::L(l) => {
                            *l += explosion.1;
                            if depth == 3 {
                                // The element that just exploded is set to zero
                                self.left = Elem::L(0);
                            }
                            return Some((explosion.0, 0));
                        }
                    }
                }
            }
            Elem::L(_) => (),
        }

        // Only check right value if the left or a child of left has not already exploded
        if let Some(left_explosion) = has_exploded {
            Some(left_explosion)
        } else {
            match &mut self.right {
                Elem::N(brackets) => {
                    if let Some(explosion) = brackets.explode(depth + 1) {
                        match &mut self.left {
                            Elem::N(bracket) => {
                                bracket.add_to_rightmost(explosion.0);
                                Some((0, explosion.1))
                            }
                            Elem::L(l) => {
                                *l += explosion.0;
                                if depth == 3 {
                                    // The element that just exploded is set to zero
                                    self.right = Elem::L(0);
                                }
                                Some((0, explosion.1))
                            }
                        }
                    } else {
                        None
                    }
                }
                Elem::L(_) => None,
            }
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
            Elem::N(brackets) => brackets.add_to_leftmost(val),
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
