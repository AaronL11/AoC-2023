advent_of_code::solution!(2);

const R: u32 = 12;
const G: u32 = 13;
const B: u32 = 14;

type Acc = (bool,u32,u32,u32);

fn f(
    (acc,r,g,b): Acc,
    (n,cube): (u32,&str)
) -> Acc {
    if acc {
        let (r,g,b) = match &cube[..cube.len()-1] {
            "red"|"re" => (r+n,g,b),
            "green"|"gree" => (r,g+n,b),
            "blue"|"blu" => (r,g,b+n),
            _ => unreachable!()
        };
            if cube.ends_with(";") || !cube.ends_with(",") {
                if r <= R && g <= G && b <= B {
                    (true,0,0,0)
                } else {
                    (false,0,0,0)
                }
            } else {
                (acc,r,g,b)
            }
    } else {
        (false,0,0,0)
    }
}

fn solve1((id, line): (usize,&str)) -> Option<u32> {
    if line.split_ascii_whitespace().skip(2).step_by(2)
    .zip(line.split_ascii_whitespace().skip(3).step_by(2))
    .map(|(n,cube)| (n.parse::<u32>().unwrap(), cube))
    .fold((true,0,0,0), f).0 {
        Some(id as u32 + 1)
    } else {
        None
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input.lines()
        .enumerate()
        .flat_map(solve1)
        .sum::<u32>()
    )
}

type Acc2 = (u32,u32,u32,u32,u32,u32);

use std::cmp::max;

fn g(
    (r,g,b,minr,ming,minb): Acc2,
    (n,cube): (u32,&str)
) -> Acc2 {
    let (r,g,b) = match &cube[..cube.len()-1] {
        "red"|"re" => (r+n,g,b),
        "green"|"gree" => (r,g+n,b),
        "blue"|"blu" => (r,g,b+n),
        _ => unreachable!()
    };
    if cube.ends_with(";") || !cube.ends_with(",") {
        (0,0,0,max(r,minr),max(g,ming),max(b,minb))
    } else {
        (r,g,b,max(r,minr),max(g,ming),max(b,minb))
    }
}

fn solve2((_id, line): (usize,&str)) -> Option<u32> {
    let (_,_,_,r,g,b) = line.split_ascii_whitespace().skip(2).step_by(2)
    .zip(line.split_ascii_whitespace().skip(3).step_by(2))
    .map(|(n,cube)| (n.parse::<u32>().unwrap(), cube))
    .fold((0,0,0,0,0,0), g);
    Some(r*g*b)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input.lines()
        .enumerate()
        .flat_map(solve2)
        .sum::<u32>()
    )
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
