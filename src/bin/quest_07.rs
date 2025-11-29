use std::collections::HashSet;

ec::solution!(7);

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<String>) {
    let (names, instructions) = input.split_once("\n\n").unwrap();

    let names = names
        .split(',')
        .map(|n| n.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let instructions = instructions
        .lines()
        .flat_map(|line| {
            line.split_once(" > ")
                .map(|(a, b)| b.split(',').map(|s| a.to_string() + s).collect::<Vec<_>>())
                .unwrap()
        })
        .collect::<Vec<_>>();
    (names, instructions)
}

fn valid_names(names: &[Vec<char>], instructions: &[String]) -> Vec<(usize, String)> {
    names
        .iter()
        .enumerate()
        .filter_map(|(i, name)| {
            if (**name)
                .windows(2)
                .all(|w| instructions.contains(&w.iter().collect::<String>()))
            {
                Some((i, name.iter().collect::<String>()))
            } else {
                None
            }
        })
        .collect()
}

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let (names, instructions) = parse(notes);

    let valid_names = valid_names(&names, &instructions);

    Some(valid_names.first().unwrap().1.to_owned())
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    let (names, instructions) = parse(notes);

    let valid_names = valid_names(&names, &instructions);

    let index_sum = valid_names.iter().map(|(i, _)| i + 1).sum::<usize>();

    Some(index_sum.to_string())
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    let (names, instructions) = parse(notes);

    let valid_names = valid_names(&names, &instructions);

    let res: HashSet<String> = valid_names
        .iter()
        .flat_map(|n| generate(&n.1, &instructions, 11))
        .collect();

    Some(res.len().to_string())
}

fn generate(name: &str, instructions: &[String], max_len: usize) -> HashSet<String> {
    let cur_len = name.len();
    let mut names = HashSet::<String>::new();
    names.insert(name.to_owned());
    let mut res = HashSet::<String>::new();
    for i in cur_len..max_len {
        let new_names = names
            .iter()
            .flat_map(|name| {
                instructions.iter().filter_map(|instruction| {
                    if instruction.starts_with(name.chars().last().unwrap()) {
                        let mut new_name = name.clone();
                        new_name.push_str(&instruction[1..]);
                        Some(new_name)
                    } else {
                        None
                    }
                })
            })
            .collect::<HashSet<_>>();

        names = new_names.clone();
        if i >= 6 {
            res.extend(new_names);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(7, 1));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(7, 2));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(7, 3));
        assert_eq!(result, None);
    }
}
