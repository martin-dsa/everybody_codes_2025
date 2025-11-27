use std::collections::{HashMap, HashSet};

ec::solution!(3);

fn get_crates(notes: &str) -> impl Iterator<Item = i32> {
    notes.split(',').map(|s| s.parse::<i32>().unwrap())
}

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let crates = get_crates(notes);
    let sum_unique: i32 = crates.collect::<HashSet<_>>().iter().sum();
    Some(sum_unique.to_string())
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    let crates = get_crates(notes).collect::<HashSet<_>>();
    let mut crates = crates.iter().copied().collect::<Vec<i32>>();

    crates.sort();

    let sum_unique_lowest_20: i32 = crates.iter().take(20).sum();
    Some(sum_unique_lowest_20.to_string())
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    let crates = get_crates(notes);
    let mut map = HashMap::<i32, i32>::new();
    for c in crates {
        *map.entry(c).or_insert(0) += 1
    }

    let most_common_crate_size = map.values().max().unwrap();
    Some(most_common_crate_size.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(3, 1));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(3, 2));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(3, 3));
        assert_eq!(result, None);
    }
}
