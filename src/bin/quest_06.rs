ec::solution!(6);

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let mut res = 0;

    for (i, c) in notes.char_indices() {
        if c == 'a' {
            let prev = &notes[0..i];
            res += prev.chars().filter(|x| *x == 'A').count();
        }
    }
    Some(res.to_string())
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    let mut res = 0;

    for (i, c) in notes.char_indices() {
        let prev = &notes[0..i];
        for ch in b'a'..=b'c' {
            if c as u8 == ch {
                res += prev.chars().filter(|x| *x as u8 == ch - 32).count();
            }
        }
    }
    Some(res.to_string())
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    let mut res = 0;
    let bytes = notes.as_bytes().repeat(1000);

    for (i, c) in bytes.iter().enumerate() {
        let prev = &bytes[i.saturating_sub(1000)..i];

        for ch in b'a'..=b'c' {
            if *c == ch {
                res += prev.iter().filter(|x| **x == ch - 32).count();
            }
        }
    }

    let reversed = notes.chars().rev().collect::<String>();

    let bytes = reversed.as_bytes().repeat(1000);

    for (i, c) in bytes.iter().enumerate() {
        let prev = &bytes[i.saturating_sub(1000)..i];
        for ch in b'a'..=b'c' {
            if *c == ch {
                res += prev.iter().filter(|x| **x == ch - 32).count();
            }
        }
    }

    Some(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(6, 1));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(6, 2));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(6, 3));
        assert_eq!(result, None);
    }
}
