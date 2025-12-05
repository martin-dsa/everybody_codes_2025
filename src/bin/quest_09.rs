use std::collections::HashSet;

use itertools::Itertools;

ec::solution!(9);

fn get_degree(parent: &str, child: &str) -> usize {
    parent
        .chars()
        .zip(child.chars())
        .filter(|(p, c)| p == c)
        .count()
}

fn are_parents(parent1: &str, parent2: &str, child: &str) -> bool {
    parent1
        .chars()
        .zip(parent2.chars())
        .zip(child.chars())
        .all(|((p1, p2), c)| p1 == c || p2 == c)
}

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let mut dnas = notes.lines().map(|line| line.split_once(':').unwrap().1);
    let parents = [dnas.next().unwrap(), dnas.next().unwrap()];
    let child = dnas.next().unwrap();

    let res = parents
        .iter()
        .map(|parent| get_degree(parent, child))
        .product::<usize>();
    Some(res.to_string())
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    let dnas = notes
        .lines()
        .map(|line| line.split_once(':').unwrap().1)
        .collect::<Vec<_>>();
    let dnas = dnas
        .iter()
        .flat_map(|dna1| {
            dnas.iter()
                .filter(move |dna2| *dna2 != dna1)
                .combinations(2)
                .map(|c| (*c[0], *c[1], *dna1))
                .filter_map(|(p1, p2, c)| {
                    if are_parents(p1, p2, c) {
                        Some(get_degree(p1, c) * get_degree(p2, c))
                    } else {
                        None
                    }
                })
        })
        .sum::<usize>();
    Some(dnas.to_string())
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    let dnas = notes
        .lines()
        .map(|line| line.split_once(':').unwrap().1)
        .collect::<Vec<_>>();

    let mut families_original = dnas
        .iter()
        .flat_map(|dna1| {
            dnas.iter()
                .filter(move |dna2| *dna2 != dna1)
                .combinations(2)
                .map(|c| (*c[0], *c[1], *dna1))
                .filter_map(|(p1, p2, c)| {
                    if are_parents(p1, p2, c) {
                        Some(HashSet::from([p1, p2, c]))
                    } else {
                        None
                    }
                })
        })
        .collect::<Vec<_>>();

    let mut families: Vec<HashSet<&str>> = vec![];

    loop {
        for fam1 in &families_original {
            if let Some(fam2) = families.iter_mut().find(|x| !x.is_disjoint(fam1)) {
                *fam2 = fam2.union(fam1).cloned().collect::<HashSet<_>>();
            } else {
                families.push(fam1.clone());
            }
        }
        if families.iter().all(|x| families_original.contains(x)) {
            break;
        }
        families_original = families.clone();
    }

    let res = families
        .iter()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
        .iter()
        .map(|a| dnas.iter().find_position(|b| *b == a).unwrap().0 + 1)
        .sum::<usize>();

    Some(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(9, 1));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(9, 2));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(9, 3));
        assert_eq!(result, None);
    }
}
