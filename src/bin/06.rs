use itertools::Itertools;

advent_of_code::solution!(6);

#[derive(Copy, Clone)]
enum Op {
    Add,
    Mul,
}

impl Op {
    fn eval(&self, nums: &[u64]) -> u64 {
        match self {
            Op::Add => nums.iter().sum(),
            Op::Mul => nums.iter().product(),
        }
    }
}

struct Problem {
    nums: Vec<u64>,
    op: Op,
}

fn parse1(input: &str) -> Vec<Problem> {
    let mut lines = input.lines().collect_vec();

    let ops = lines
        .remove(lines.len() - 1)
        .split_ascii_whitespace()
        .map(|s| match s {
            "+" => Op::Add,
            "*" => Op::Mul,
            _ => panic!(),
        })
        .collect_vec();

    let nums = lines
        .iter()
        .map(|s| {
            s.split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let mut problems = vec![];
    for i in 0..nums[0].len() {
        let nums = nums.iter().map(|n| n[i]).collect_vec();
        let op = ops[i];

        problems.push(Problem { nums, op });
    }

    problems
}

pub fn part_one(input: &str) -> Option<u64> {
    let problems = parse1(input);
    let ans = problems.iter().map(|p| p.op.eval(&p.nums)).sum();

    Some(ans)
}

fn parse2(input: &str) -> Vec<Problem> {
    let mut lines = input.lines().collect_vec();

    let ops = lines
        .remove(lines.len() - 1)
        .split_ascii_whitespace()
        .map(|s| match s {
            "+" => Op::Add,
            "*" => Op::Mul,
            _ => panic!(),
        })
        .collect_vec();

    let mut problems = ops
        .iter()
        .map(|&op| Problem { nums: vec![], op })
        .collect_vec();
    let mut problem_i = 0;

    for c in 0..lines[0].len() {
        let mut current_col = vec![];
        for r in &lines {
            current_col.push(r.chars().nth(c).unwrap());
        }

        let current_col: String = current_col.into_iter().collect();
        let current_col = current_col.trim();

        if current_col.is_empty() {
            problem_i += 1;
            continue;
        } else {
            problems[problem_i].nums.push(current_col.parse().unwrap());
        }
    }

    problems
}

pub fn part_two(input: &str) -> Option<u64> {
    let problems = parse2(input);
    let ans = problems.iter().map(|p| p.op.eval(&p.nums)).sum();

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
