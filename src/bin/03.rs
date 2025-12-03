use itertools::Itertools;

advent_of_code::solution!(3);

fn parse(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect_vec()
        })
        .collect_vec()
}

fn first_max_index(v: &[u64]) -> Option<usize> {
    let max = v.iter().max()?;

    v.iter().position(|x| x == max)
}

fn highest_joltage(bank: &[u64], digits: usize) -> u64 {
    let next_digit = first_max_index(&bank[..bank.len() - digits + 1]).unwrap();

    if digits == 1 {
        bank[next_digit]
    } else {
        bank[next_digit] * 10u64.pow((digits - 1) as u32)
            + highest_joltage(&bank[next_digit + 1..], digits - 1)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse(input);

    let ans: u64 = input.iter().map(|b| highest_joltage(b, 2)).sum();

    Some(ans as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse(input);

    let ans: u64 = input.iter().map(|b| highest_joltage(b, 12)).sum();

    Some(ans as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
