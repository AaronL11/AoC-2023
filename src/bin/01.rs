advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input.lines()
        .map(|l| {
            let (a,b) = l.chars()
            .filter(char::is_ascii_digit)
            .zip(
                l.chars().rev()
                .filter(char::is_ascii_digit)
            )
            .next().unwrap_or(('0','0'));
            (a as u32 - 48)*10+(b as u32 - 48)
        })
        .sum::<u32>()
    )
}

fn f1(
    (mut stack, mut last) : (Vec<u32>,String),
    n: char
) -> (Vec<u32>,String) {
    if n.is_ascii_digit() {
        stack.push(n as u32 - 48);
        last.clear();
    } else {
        if let Some(x) = match n {
            'e' if last.ends_with("on") => Some(1),
            'o' if last.ends_with("tw") => Some(2),
            'e' if last.ends_with("thre") => Some(3),
            'r' if last.ends_with("fou") => Some(4),
            'e' if last.ends_with("fiv") => Some(5),
            'x' if last.ends_with("si") => Some(6),
            'n' if last.ends_with("seve") => Some(7),
            't' if last.ends_with("eigh") => Some(8),
            'e' if last.ends_with("nin") => Some(9),
            _ => None
        } {
            stack.push(x);
            last.clear();
        } else {
            last.push(n);
        }
    };
    (stack,last)
}

fn f2(
    (mut stack, mut last) : (Vec<u32>,String),
    n: char
) -> (Vec<u32>,String) {
    if n.is_ascii_digit() {
        stack.push(n as u32 - 48);
        last.clear();
    } else {
        if let Some(x) = match n {
            'o' if last.ends_with("en") => Some(1),
            't' if last.ends_with("ow") => Some(2),
            't' if last.ends_with("eerh") => Some(3),
            'f' if last.ends_with("ruo") => Some(4),
            'f' if last.ends_with("evi") => Some(5),
            's' if last.ends_with("xi") => Some(6),
            's' if last.ends_with("neve") => Some(7),
            'e' if last.ends_with("thgi") => Some(8),
            'n' if last.ends_with("eni") => Some(9),
            _ => None
        } {
            stack.push(x);
            last.clear();
        } else {
            last.push(n);
        }
    };
    (stack,last)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input.lines()
        .map(|l|
            (l.chars(),l.chars().rev())
        )
        .map(|(iter1,iter2)|
            (iter1.fold((vec![],String::new()), f1),
            iter2.fold((vec![],String::new()), f2))
        )
        .map(|((stack1,_),(stack2,_))| (stack1,stack2))
        .map(|(stack1,stack2)| stack1[0]*10 + stack2[0])
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
