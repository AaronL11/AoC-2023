use std::collections::HashMap;

advent_of_code::solution!(3);

const DIRS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (0, -1),
];

#[allow(non_snake_case)]
pub fn part_one(input: &str) -> Option<u32> {
    let M = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();
    let mut b = false;
    let mut b2 = false;
    let mut stack = vec![];
    let mut sum = 0;
    for (i, j) in (0..M.len()).flat_map(|i| std::iter::once(i).cycle().zip(0..M[0].len())) {
        let x = M[i][j];
        if x.is_ascii_digit() {
            if b {
                let y = stack.pop()?;
                stack.push(y * 10 + (x - 48) as u32);
            } else {
                stack.push((x - 48) as u32);
            }
            for (dx, dy) in DIRS
                .iter()
                .map(|(a, b)| (i as isize + a, j as isize + b))
                .filter(|(a, b)| *a >= 0 && *b >= 0)
                .map(|(a, b)| (a as usize, b as usize))
            {
                let b3 = (dx, dy) == (i, j + 1);
                if let Some(row) = M.get(dx) {
                    if let Some(y) = row.get(dy) {
                        if b3 {
                            b = if y.is_ascii_digit() { true } else { false }
                        }
                        if !y.is_ascii_digit() && *y != b'.' {
                            b2 = true;
                        }
                    };
                }
            }
            if j == M[0].len() - 1 {
                b = false
            }
            if b2 && !b {
                sum += stack.pop()? as u32;
                b2 = false;
            }
        } else {
            b = false;
        }
    }
    Some(sum)
}

#[allow(non_snake_case)]
pub fn part_two(input: &str) -> Option<u32> {
    let mut gears: HashMap<(usize, usize), (u32, u32)> = HashMap::new();
    let M = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();
    let mut b = false;
    let mut b2 = false;
    let mut stack = vec![];
    let mut gear = (0, 0);
    for (i, j) in (0..M.len()).flat_map(|i| std::iter::once(i).cycle().zip(0..M[0].len())) {
        let x = M[i][j];
        if x.is_ascii_digit() {
            if b {
                let y = stack.pop()?;
                stack.push(y * 10 + (x - 48) as u32);
            } else {
                stack.push((x - 48) as u32);
            }
            for (dx, dy) in DIRS
                .iter()
                .map(|(a, b)| (i as isize + a, j as isize + b))
                .filter(|(a, b)| *a >= 0 && *b >= 0)
                .map(|(a, b)| (a as usize, b as usize))
            {
                let b3 = (dx, dy) == (i, j + 1);
                if let Some(row) = M.get(dx) {
                    if let Some(y) = row.get(dy) {
                        if b3 {
                            b = if y.is_ascii_digit() { true } else { false }
                        }
                        if *y == b'*' {
                            gear = (dx, dy);
                            b2 = true;
                        }
                    };
                }
            }
            if j == M[0].len() - 1 {
                b = false
            }
            if b2 && !b {
                if let Some((n, r)) = gears.get_mut(&gear) {
                    *n += 1;
                    *r *= stack.pop()?
                } else {
                    gears.insert(gear, (1, stack.pop()?));
                }
                gear = (0, 0);
                b2 = false;
            }
        } else {
            b = false;
        }
    }
    gears
        .values()
        .filter_map(|(n, r)| if *n == 2 { Some(*r) } else { None })
        .reduce(|a, b| a + b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
