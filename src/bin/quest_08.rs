use std::f64::consts::PI;

ec::solution!(8);

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    let pairs = input
        .split(',')
        .map(|line| line.parse().unwrap())
        .collect::<Vec<usize>>();

    pairs
        .windows(2)
        .map(|x| (x[0].min(x[1]), x[0].max(x[1])))
        .collect()
}

fn get_points_coordinates(size: usize) -> Vec<(f64, f64)> {
    let center = (0f64, 0f64);
    let radius = size;
    let mut points = Vec::new();
    for i in 0..size {
        let a = PI * 2.0 / size as f64 * i as f64;
        let x = center.0 + radius as f64 * a.cos();
        let y = center.0 + radius as f64 * a.sin();
        points.push((x, y));
    }

    points
}

fn lines_intersect(l1: ((f64, f64), (f64, f64)), l2: ((f64, f64), (f64, f64))) -> bool {
    if l1.0 == l2.0 || l1.0 == l2.1 || l1.1 == l2.0 || l1.1 == l2.1 {
        return false;
    }

    let ((x1, y1), (x2, y2)) = l1;
    let ((x3, y3), (x4, y4)) = l2;

    let r_x = x2 - x1;
    let r_y = y2 - y1;
    let s_x = x4 - x3;
    let s_y = y4 - y3;

    let denom = r_x * s_y - r_y * s_x;
    if denom.abs() < 1e-12 {
        return false;
    }

    let qp_x = x3 - x1;
    let qp_y = y3 - y1;

    let t = (qp_x * s_y - qp_y * s_x) / denom;
    let u = (qp_x * r_y - qp_y * r_x) / denom;

    (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u)
}

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    const SIZE: usize = 32;
    let sequence = parse_input(notes);

    let res = sequence
        .iter()
        .filter(|x| x.0.abs_diff(x.1) == SIZE / 2)
        .count();

    Some(res.to_string())
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    const SIZE: u32 = 256;
    let sequence = parse_input(notes);
    let points = get_points_coordinates(SIZE as usize);

    let mut res = 0;
    for (i, new_thread) in sequence.iter().enumerate() {
        let l1 = (
            *points.get(new_thread.0 - 1).unwrap(),
            *points.get(new_thread.1 - 1).unwrap(),
        );
        for existing_thread in &sequence[0..i] {
            let l2 = (
                *points.get(existing_thread.0 - 1).unwrap(),
                *points.get(existing_thread.1 - 1).unwrap(),
            );

            if lines_intersect(l1, l2) {
                res += 1
            }
        }
    }

    Some(res.to_string())
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    const SIZE: u32 = 256;
    let sequence = parse_input(notes);
    let points = get_points_coordinates(SIZE as usize);

    let threads_coordinates = sequence
        .iter()
        .map(|(x, y)| (*points.get(x - 1).unwrap(), *points.get(y - 1).unwrap()))
        .collect::<Vec<_>>();

    let res = (1..SIZE)
        .flat_map(|i| {
            let value = points.clone();
            let tc = threads_coordinates.clone();
            (i..=SIZE).map(move |j| {
                let cut = (
                    *value.get(i as usize - 1).unwrap(),
                    *value.get(j as usize - 1).unwrap(),
                );
                tc.iter().filter(|c| lines_intersect(**c, cut)).count()
            })
        })
        .max()
        .unwrap();
    
    Some(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(8, 1));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(8, 2));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(8, 3));
        assert_eq!(result, None);
    }
}
