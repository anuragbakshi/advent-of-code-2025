use itertools::Itertools;
use std::collections::HashSet;

advent_of_code::solution!(8);

struct Pos {
    x: u64,
    y: u64,
    z: u64,
}

impl Pos {
    fn sq_dist(&self, other: &Pos) -> u64 {
        self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)
    }
}

fn parse(input: &str) -> Vec<Pos> {
    input
        .lines()
        .map(|s| {
            let (x, y, z) = s
                .split(",")
                .map(|i| i.parse().unwrap())
                .collect_tuple()
                .unwrap();

            Pos { x, y, z }
        })
        .collect_vec()
}

fn sq_distance_matrix(pos: &[Pos]) -> Vec<Vec<u64>> {
    pos.iter()
        .map(|p| pos.iter().map(|q| p.sq_dist(q)).collect_vec())
        .collect_vec()
}

fn find_component(components: &[HashSet<usize>], a: usize) -> usize {
    components.iter().position(|c| c.contains(&a)).unwrap()
}

fn merge_components(components: &mut Vec<HashSet<usize>>, a: usize, b: usize) {
    let a_i = find_component(components, a);
    let b_i = find_component(components, b);

    if a_i != b_i {
        let removed = components.swap_remove(a_i.max(b_i));
        components[a_i.min(b_i)].extend(removed);
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let points = parse(input);
    let dist = sq_distance_matrix(&points);

    let points_by_dist = (0..points.len())
        .tuple_combinations()
        .sorted_by_key(|&(i, j)| dist[i][j])
        .collect_vec();

    let mut components = (0..points.len()).map(|x| HashSet::from([x])).collect_vec();

    points_by_dist
        .iter()
        .take(1000)
        .for_each(|&(i, j)| merge_components(&mut components, i, j));

    let ans: usize = components
        .iter()
        .map(|c| c.len())
        .sorted()
        .rev()
        .take(3)
        .product();

    Some(ans as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let points = parse(input);
    let dist = sq_distance_matrix(&points);

    let points_by_dist = (0..points.len())
        .tuple_combinations()
        .sorted_by_key(|&(i, j)| dist[i][j])
        .collect_vec();

    let mut components = (0..points.len()).map(|x| HashSet::from([x])).collect_vec();

    for &(i, j) in &points_by_dist {
        merge_components(&mut components, i, j);

        if components.len() == 1 {
            return Some(points[i].x * points[j].x);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
