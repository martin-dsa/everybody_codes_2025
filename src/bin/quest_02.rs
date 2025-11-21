use std::{
    convert::Infallible,
    fmt::Display,
    ops::{Add, Div, Mul},
    str::FromStr,
};

ec::solution!(2);

#[derive(Debug, Clone, Copy)]
struct ComplexNumber(i64, i64);
impl FromStr for ComplexNumber {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .trim_start_matches("A=[")
            .trim_end_matches("]")
            .split_once(',')
            .map(|(x, y)| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()))
            .unwrap();
        Ok(ComplexNumber(x, y))
    }
}

impl Display for ComplexNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.0, self.1)
    }
}

impl Add for ComplexNumber {
    type Output = Self;

    fn add(self, other: ComplexNumber) -> ComplexNumber {
        ComplexNumber(self.0 + other.0, self.1 + other.1)
    }
}

impl Mul for ComplexNumber {
    type Output = Self;

    fn mul(self, other: ComplexNumber) -> ComplexNumber {
        ComplexNumber(
            self.0 * other.0 - self.1 * other.1,
            self.0 * other.1 + self.1 * other.0,
        )
    }
}

impl Div for ComplexNumber {
    type Output = Self;

    fn div(self, other: ComplexNumber) -> Self::Output {
        ComplexNumber(self.0 / other.0, self.1 / other.1)
    }
}

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let a = notes.parse::<ComplexNumber>().unwrap();
    let mut res = ComplexNumber(0, 0);

    for i in 0..3 {
        res = res * res;
        res = res / ComplexNumber(10, 10);
        res = res + a;
    }

    Some(res.to_string())
}

fn engraved_points(start_pos: ComplexNumber, step: usize) -> impl Iterator<Item = ComplexNumber> {
    let end_pos = start_pos + ComplexNumber(1000, 1000);

    (start_pos.0..=end_pos.0).step_by(step).flat_map(move |x| {
        (start_pos.1..=end_pos.1)
            .step_by(step)
            .filter_map(move |y| {
                let n = ComplexNumber(x, y);
                let mut res = ComplexNumber(0, 0);

                for _ in 0..100 {
                    res = res * res;
                    res = res / ComplexNumber(100_000, 100_000);
                    res = res + n;

                    if res.0 > 1_000_000
                        || res.0 < -1_000_000
                        || res.1 > 1_000_000
                        || res.1 < -1_000_000
                    {
                        return None;
                    }
                }
                Some(res)
            })
    })
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    let start_pos = notes.parse::<ComplexNumber>().unwrap();
    let points = engraved_points(start_pos, 10);
    Some(points.count().to_string())
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    let start_pos = notes.parse::<ComplexNumber>().unwrap();
    let points = engraved_points(start_pos, 1);
    Some(points.count().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(2, 1));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(2, 2));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(2, 3));
        assert_eq!(result, None);
    }
}
