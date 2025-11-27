ec::solution!(4);

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let gears = notes
        .lines()
        .map(|line| line.parse::<f64>().unwrap())
        .collect::<Vec<f64>>();

    let total_ratio = gears.windows(2).map(|x| x[0] / x[1]).product::<f64>();

    Some(((total_ratio * 2025.0) as u64).to_string())
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    let gears = notes
        .lines()
        .map(|line| line.parse::<f64>().unwrap())
        .collect::<Vec<f64>>();

    let total_ratio = gears.windows(2).map(|x| x[0] / x[1]).product::<f64>();

    Some(((10000000000000.0 / total_ratio) as u64 + 1).to_string())
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    let gears = notes
        .lines()
        .flat_map(|line| match line.split_once('|') {
            None => vec![line.parse::<f64>().unwrap()],
            Some((a, b)) => vec![a.parse::<f64>().unwrap(), b.parse::<f64>().unwrap()],
        })
        .collect::<Vec<f64>>();

    let total_ratio = gears.chunks(2).map(|x| x[0] / x[1]).product::<f64>();

    Some(((total_ratio * 100.0) as u64).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(4, 1));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(4, 2));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(4, 3));
        assert_eq!(result, None);
    }
}
