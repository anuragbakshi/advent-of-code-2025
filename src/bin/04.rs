use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(4);

#[derive(Hash, Eq, PartialEq)]
struct Pos(isize, isize);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Paper,
    Empty,
}

type Grid = Vec<Vec<Cell>>;

impl Pos {
    fn neighbours(&self, grid: &Grid) -> HashSet<Pos> {
        (-1..=1)
            .cartesian_product(-1..=1)
            .map(|(x, y)| Pos(self.0 + x, self.1 + y))
            .filter(|p| p != self)
            .filter(|&Pos(x, y)| {
                0 <= x && x < grid.len() as isize && 0 <= y && y < grid[0].len() as isize
            })
            .collect()
    }

    fn at(&self, grid: &Grid) -> Cell {
        grid[self.0 as usize][self.1 as usize]
    }
}

fn is_accessible(pos: &Pos, grid: &Grid) -> bool {
    let paper_neighbors = pos
        .neighbours(grid)
        .iter()
        .filter(|p| p.at(grid) == Cell::Paper)
        .count();

    paper_neighbors < 4
}

fn parse(input: &str) -> Grid {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| if c == '@' { Cell::Paper } else { Cell::Empty })
                .collect_vec()
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse(input);

    let ans = (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .filter(|&(x, y)| {
            let pos = Pos(x as isize, y as isize);
            pos.at(&grid) == Cell::Paper && is_accessible(&pos, &grid)
        })
        .count() as u64;

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = parse(input);

    let mut ans = 0;

    loop {
        let mut updated = false;
        for x in 0..grid.len() {
            for y in 0..grid[0].len() {
                let pos = Pos(x as isize, y as isize);
                if pos.at(&grid) == Cell::Paper && is_accessible(&pos, &grid) {
                    grid[x][y] = Cell::Empty;
                    updated = true;
                    ans += 1;
                }
            }
        }

        if !updated {
            break;
        }
    }

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
