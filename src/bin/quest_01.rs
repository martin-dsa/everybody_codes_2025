use std::panic;

ec::solution!(1);

fn parse_input(input: &str) -> (Vec<&str>, Vec<i32>) {
    let mut lines = input
        .split("\n\n")
        .map(|l| l.split(',').collect::<Vec<&str>>());
    let names = lines.next().unwrap();
    let instructions = lines.next().unwrap();
    let instructions = instructions
        .iter()
        .map(|i| match i.as_bytes() {
            [b'R', n @ ..] => std::str::from_utf8(n).unwrap().parse::<i32>().unwrap(),
            [b'L', n @ ..] => -(std::str::from_utf8(n).unwrap().parse::<i32>().unwrap()),
            _ => panic!("Invalid instruction"),
        })
        .collect();
    (names, instructions)
}

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let (names, instructions) = parse_input(notes);
    let name = names[instructions
        .iter()
        .fold(0, |acc, x| (acc + x).max(0).min((names.len() - 1) as i32))
        as usize];
    Some(name.to_string())
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    let (names, instructions) = parse_input(notes);
    let name = names[(instructions.iter().sum::<i32>() as usize).rem_euclid(names.len())];
    Some(name.to_string())
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    let (mut names, instructions) = parse_input(notes);
    for i in instructions {
        let index_to_swap = i.rem_euclid(names.len() as i32) as usize;
        names.swap(0, index_to_swap);
    }
    let name = names[0];
    Some(name.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(1, 1));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(1, 2));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(1, 3));
        assert_eq!(result, None);
    }
}
