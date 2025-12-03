use itertools::Itertools;

advent_of_code::solution!(2);

fn parse(input: &str) -> Vec<(u64, u64)> {
    input
        .split(",")
        .map(|x| {
            x.split('-')
                .map(|n| n.trim().parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u64> {
    fn is_invalid(n: u64) -> bool {
        let n = n.to_string();
        let l = n.len();

        l.is_multiple_of(2) && n[..l / 2] == n[l / 2..]
    }

    let input = parse(input);

    let mut invalid_ids = 0;
    for (low, high) in input {
        for n in low..=high {
            if is_invalid(n) {
                invalid_ids += n;
            }
        }
    }

    Some(invalid_ids)
}

pub fn part_two(input: &str) -> Option<u64> {
    fn is_invalid(n: u64) -> bool {
        let n = n.to_string();
        let l = n.len();

        (1..=(l / 2)).any(|d| {
            l.is_multiple_of(d)
                && n.chars()
                    .chunks(d)
                    .into_iter()
                    .map(|x| x.collect_vec())
                    .all_equal()
        })
    }

    let input = parse(input);

    let mut invalid_ids = 0;
    for (low, high) in input {
        for n in low..=high {
            if is_invalid(n) {
                invalid_ids += n;
            }
        }
    }

    Some(invalid_ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
