use itertools::Itertools;

advent_of_code::solution!(5);

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Range(usize, usize);

impl Range {
    fn contains(&self, x: usize) -> bool {
        self.0 <= x && x <= self.1
    }

    fn merge_all(ranges: &[Self]) -> Vec<Self> {
        let mut acc = vec![*ranges.first().unwrap()];

        for &r in ranges {
            let last = acc.last_mut().unwrap();
            if last.contains(r.0) {
                last.1 = last.1.max(r.1);
            } else {
                acc.push(r);
            }
        }

        acc
    }

    fn length(&self) -> usize {
        self.1 - self.0 + 1
    }
}

fn parse(input: &str) -> (Vec<Range>, Vec<usize>) {
    let mut lines = input.lines();

    let mut ranges = vec![];
    loop {
        let next = lines.next().unwrap();
        if next.is_empty() {
            break;
        }

        let (start, end) = next
            .split("-")
            .map(|x| x.parse().unwrap())
            .collect_tuple()
            .unwrap();

        ranges.push(Range(start, end));
    }

    ranges.sort();

    let available = lines.map(|x| x.parse().unwrap()).collect_vec();

    (ranges, available)
}

fn ranges_contain(ranges: &[Range], x: usize) -> bool {
    ranges.iter().any(|r| r.contains(x))
}

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, available) = parse(input);

    let ans = available
        .iter()
        .filter(|&&a| ranges_contain(&ranges, a))
        .count();

    Some(ans as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges, _) = parse(input);

    let ranges = Range::merge_all(&ranges);
    let ans: usize = ranges.iter().map(|r| r.length()).sum();

    Some(ans as u64)
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
        assert_eq!(result, Some(14));
    }
}
