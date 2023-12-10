use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(4);

fn solve(line: &str) -> u32 {
    let (mut b, mut set, mut cnt) = (false, HashSet::new(), 0);
    line.split_ascii_whitespace().skip(2).for_each(|c| {
        if c != "|" {
            if b {
                if set.contains(&c) {
                    cnt += 1;
                }
            } else {
                set.insert(c);
            }
        } else {
            b = true;
        }
    });
    if cnt > 0 {
        1 << (cnt - 1)
    } else {
        cnt
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    input.lines().map(solve).sum1()
}

fn solve_2(line: &str) -> u32 {
    let (mut b, mut set, mut cnt) = (false, HashSet::new(), 0);
    line.split_ascii_whitespace().skip(2).for_each(|c| {
        if c != "|" {
            if b {
                if set.contains(&c) {
                    cnt += 1;
                }
            } else {
                set.insert(c);
            }
        } else {
            b = true;
        }
    });
    if cnt > 0 {
        1 << (cnt - 1)
    } else {
        cnt
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut streak = HashMap::new();
    input
        .lines()
        .map(solve_2)
        .enumerate()
        .fold(0, |sum, (i, cnt)| {
            (1..=cnt as usize).for_each(|j| {
                streak
                    .entry(i + 1 + j)
                    .and_modify(|v| *v += cnt)
                    .or_insert(cnt);
            });
            dbg!(i, &streak);
            if let Some(x) = streak.get(&(i + 1)) {
                sum + cnt * x
            } else {
                sum + cnt
            }
        })
        .into()
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
