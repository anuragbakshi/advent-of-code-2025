use itertools::Itertools;

advent_of_code::solution!(1);

fn parse(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|l| {
            let m = match l.chars().next().unwrap() {
                'L' => -1,
                'R' => 1,
                _ => panic!(),
            };

            let n: i64 = l[1..].parse().unwrap();

            m * n
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse(input);

    let mut times_at_0 = 0;
    let mut n = 50;

    for i in input {
        n = (n + i + 100) % 100;

        if n == 0 {
            times_at_0 += 1;
        }
    }

    Some(times_at_0)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse(input);

    let mut times_at_0 = 0;
    let mut n = 50;

    for i in input {
        let next_n = n + i;

        times_at_0 += n.div_euclid(100).abs_diff(next_n.div_euclid(100));

        n = next_n;
    }

    Some(times_at_0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
