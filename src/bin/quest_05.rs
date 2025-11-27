use std::{cmp::Ordering, convert::Infallible, str::FromStr};

ec::solution!(5);
#[derive(Debug, PartialEq, Eq)]
struct Spine {
    segments: Box<SpineSegment>,
    id: i32,
}

impl Spine {
    fn quality(&self) -> u64 {
        let mut res: String = String::from("");
        let mut segments = &self.segments;
        while segments.center.is_some() {
            res += segments.center.unwrap().to_string().as_str();
            if let Some(next) = segments.next.as_ref() {
                segments = next;
            } else {
                break;
            }
        }

        res.parse::<u64>().unwrap()
    }
}

impl FromStr for Spine {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, s) = s.split_once(':').unwrap();
        let id = id.parse::<i32>().unwrap();

        let values = s.split(',').map(|x| x.parse::<i32>().unwrap());
        let mut segments = Box::new(SpineSegment::new());
        for value in values {
            segments.add_segment(value);
        }

        Ok(Spine { id, segments })
    }
}

impl PartialOrd for Spine {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Spine {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.quality()
            .cmp(&other.quality())
            .then(self.segments.cmp(&other.segments))
            .then(self.id.cmp(&other.id))
    }
}

#[derive(Debug, Eq, PartialEq)]
struct SpineSegment {
    left: Option<i32>,
    center: Option<i32>,
    right: Option<i32>,
    next: Option<Box<SpineSegment>>,
}

impl SpineSegment {
    fn new() -> Self {
        Self {
            left: None,
            center: None,
            right: None,
            next: None,
        }
    }

    fn quality(&self) -> i32 {
        [self.left, self.center, self.right]
            .into_iter()
            .flatten()
            .map(|v| v.to_string())
            .collect::<String>()
            .parse()
            .unwrap_or(0)
    }

    fn add_segment(&mut self, value: i32) {
        if self.center.is_none() {
            self.center = Some(value);
            return;
        }
        
        if self.left.is_none() && self.center.unwrap() > value {
            self.left = Some(value);
            return;
        }
        
        if self.right.is_none() && self.center.unwrap() < value {
            self.right = Some(value);
            return;
        }
        
        if self.next.is_none() {
            let new_segment = Box::new(SpineSegment::new());
            self.next = Some(new_segment);
        }
        
        self.next.as_mut().unwrap().add_segment(value);
    }
}

impl PartialOrd for SpineSegment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SpineSegment {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.quality() == other.quality() {
            self.next.cmp(&other.next)
        } else {
            self.quality().cmp(&other.quality())
        }
    }
}

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let spine = notes.parse::<Spine>().unwrap();
    Some(spine.quality().to_string())
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    let spines = notes.lines().map(|l| l.parse::<Spine>().unwrap());
    let qualities = spines.map(|s| s.quality()).collect::<Vec<_>>();
    let max = qualities.iter().max().unwrap();
    let min = qualities.iter().min().unwrap();
    Some((max - min).to_string())
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    let spines = notes.lines().map(|l| l.parse::<Spine>().unwrap());
    let mut spines = spines.collect::<Vec<_>>();
    spines.sort_by(|a, b| b.cmp(a));

    let checksum = spines
        .iter()
        .map(|s| s.id)
        .enumerate()
        .map(|(i, n)| (i + 1) as i32 * n)
        .sum::<i32>();
    Some(checksum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(5, 1));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(5, 2));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(5, 3));
        assert_eq!(result, None);
    }
}
