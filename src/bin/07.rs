use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(7);

enum Cell {
    Empty,
    Splitter,
}

fn parse(input: &str) -> (usize, Vec<Vec<Cell>>) {
    let mut lines = input.lines();
    let (start, _) = lines
        .next()
        .unwrap()
        .char_indices()
        .find(|&(_, c)| c == 'S')
        .unwrap();

    let cells = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    if c == '^' {
                        Cell::Splitter
                    } else {
                        Cell::Empty
                    }
                })
                .collect_vec()
        })
        .collect_vec();

    (start, cells)
}

fn apply_layer(
    beams: &HashMap<usize, usize>,
    cells: &[Cell],
    splits: &mut u64,
) -> HashMap<usize, usize> {
    let mut next_layer = HashMap::new();
    for (&b, &num) in beams.iter() {
        match cells[b] {
            Cell::Empty => *next_layer.entry(b).or_default() += num,
            Cell::Splitter => {
                *splits += 1;
                *next_layer.entry(b - 1).or_default() += num;
                *next_layer.entry(b + 1).or_default() += num;
            }
        }
    }

    next_layer
}

pub fn part_one(input: &str) -> Option<u64> {
    let (start, cells) = parse(input);

    let mut splits = 0;
    let mut beams = HashMap::from([(start, 1)]);
    for layer in &cells {
        beams = apply_layer(&beams, layer, &mut splits);
    }

    Some(splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (start, cells) = parse(input);

    let mut splits = 0;
    let mut beams = HashMap::from([(start, 1)]);
    for layer in &cells {
        beams = apply_layer(&beams, layer, &mut splits);
    }

    Some(beams.into_values().sum::<usize>() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
