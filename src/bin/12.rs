advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    let mut caves = Vec::<Cave>::new();

    input
        .lines()
        .map(|line| line.trim().split_once('-').unwrap())
        .for_each(|(left, right)| {
            if !caves.iter().any(|cave| cave.name.eq(left)) {
                caves.push(Cave {
                    name: left.to_owned(),
                    paths: Vec::new(),
                });
            }

            if !caves.iter().any(|cave| cave.name.eq(right)) {
                caves.push(Cave {
                    name: right.to_owned(),
                    paths: Vec::new(),
                });
            }

            let left_cave = caves.iter_mut().find(|cave| cave.name.eq(left)).unwrap();
            left_cave.paths.push(right.to_owned());
            let right_cave = caves.iter_mut().find(|cave| cave.name.eq(right)).unwrap();
            right_cave.paths.push(left.to_owned());
        });

    // println!("{caves:?}");

    let start = caves.iter().find(|cave| cave.name.eq("start")).unwrap();

    let visited = Vec::new();

    Some(start.count_paths_til_end(&caves, visited))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut caves = Vec::<Cave>::new();

    input
        .lines()
        .map(|line| line.trim().split_once('-').unwrap())
        .for_each(|(left, right)| {
            if !caves.iter().any(|cave| cave.name.eq(left)) {
                caves.push(Cave {
                    name: left.to_owned(),
                    paths: Vec::new(),
                });
            }

            if !caves.iter().any(|cave| cave.name.eq(right)) {
                caves.push(Cave {
                    name: right.to_owned(),
                    paths: Vec::new(),
                });
            }

            let left_cave = caves.iter_mut().find(|cave| cave.name.eq(left)).unwrap();
            left_cave.paths.push(right.to_owned());
            let right_cave = caves.iter_mut().find(|cave| cave.name.eq(right)).unwrap();
            right_cave.paths.push(left.to_owned());
        });

    // println!("{caves:?}");

    let start = caves.iter().find(|cave| cave.name.eq("start")).unwrap();

    let visited = Vec::new();

    Some(start.count_paths_til_end_part_2(&caves, visited, false))
}

#[derive(Debug)]
struct Cave {
    name: String,
    paths: Vec<String>,
}

impl Cave {
    fn count_paths_til_end(&self, caves: &Vec<Cave>, already_visited_smalls: Vec<String>) -> u64 {
        let mut new_visit_list = already_visited_smalls.clone();
        new_visit_list.push(self.name.clone());
        if self.name.eq("end") {
            // println!("{new_visit_list:?}");
            1
        } else {
            self.paths
                .iter()
                .map(|path| {
                    let next_cave = caves.iter().find(|cave| cave.name.eq(path)).unwrap();
                    if !next_cave.name.eq(&next_cave.name.to_lowercase())
                        || !already_visited_smalls.contains(&next_cave.name)
                    {
                        next_cave.count_paths_til_end(caves, new_visit_list.clone())
                    } else {
                        0
                    }
                })
                .sum::<u64>()
        }
    }

    fn count_paths_til_end_part_2(
        &self,
        caves: &Vec<Cave>,
        already_visited_smalls: Vec<String>,
        has_used_extra_time: bool,
    ) -> u64 {
        let mut new_visit_list = already_visited_smalls.clone();
        new_visit_list.push(self.name.clone());
        if self.name.eq("end") {
            // println!("{new_visit_list:?}");
            1
        } else {
            self.paths
                .iter()
                .filter(|&path| !path.eq("start"))
                .map(|path| {
                    let next_cave = caves.iter().find(|cave| cave.name.eq(path)).unwrap();
                    if !next_cave.name.eq(&next_cave.name.to_lowercase())
                        || !already_visited_smalls.contains(&next_cave.name)
                    {
                        next_cave.count_paths_til_end_part_2(
                            caves,
                            new_visit_list.clone(),
                            has_used_extra_time,
                        )
                    } else if !has_used_extra_time {
                        next_cave.count_paths_til_end_part_2(caves, new_visit_list.clone(), true)
                    } else {
                        0
                    }
                })
                .sum::<u64>()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(226));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3509));
    }
}
