use std::collections::HashMap;

use advent2022::read_input;
use anyhow::{anyhow, Result};

type Coord = (usize, usize);
type Tree = u32;

struct Grid {
    grid: std::collections::HashMap<Coord, Tree>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn parse(input: &str) -> Result<Self> {
        let mut grid = HashMap::new();

        let lines = input.lines().enumerate();
        for (y, line) in lines {
            let chars = line.chars().enumerate();
            for (x, c) in chars {
                let coord = (x, y);
                let tree = c
                    .to_digit(10)
                    .ok_or_else(|| anyhow!("Invalid tree: {}", c))?;

                grid.insert(coord, tree);
            }
        }

        let width = grid
            .keys()
            .map(|(x, _)| *x)
            .max()
            .ok_or_else(|| anyhow!("Empty grid?"))?;

        let height = grid
            .keys()
            .map(|(_, y)| *y)
            .max()
            .ok_or_else(|| anyhow!("Empty grid?"))?;

        Ok(Self {
            grid,
            width: width + 1,
            height: height + 1,
        })
    }

    fn visible(&self, x: usize, y: usize) -> bool {
        let tree = self.get(x, y);
        let up = (0..y).all(|n| self.get(x, n) < tree);
        let down = (y + 1..self.height).all(|n| self.get(x, n) < tree);
        let left = (0..x).all(|n| self.get(n, y) < tree);
        let right = (x + 1..self.width).all(|n| self.get(n, y) < tree);

        up || down || left || right
    }
    fn score(&self, x: usize, y: usize) -> usize {
        let up = (0..y)
            .rev()
            .take_while(|n| *n == y - 1 || self.get(x, *n + 1) < self.get(x, y))
            .count();

        let right = (x + 1..self.width)
            .take_while(|n| *n == x + 1 || self.get(*n - 1, y) < self.get(x, y))
            .count();

        let down = (y + 1..self.height)
            .take_while(|n| *n == y + 1 || self.get(x, *n - 1) < self.get(x, y))
            .count();

        let left = (0..x)
            .rev()
            .take_while(|n| *n == x - 1 || self.get(*n + 1, y) < self.get(x, y))
            .count();

        up * down * left * right
    }

    fn get(&self, x: usize, y: usize) -> Tree {
        self.grid.get(&(x, y)).cloned().unwrap_or(0)
    }

    pub fn all(&self) -> impl Iterator<Item = Coord> + '_ {
        (0..self.height)
            .map(move |y| (0..self.width).map(move |x| (x, y)))
            .flatten()
    }

    pub fn max_score(&self) -> usize {
        self.all()
            .map(|(x, y)| self.score(x, y))
            .max()
            .expect("Empty grid")
    }
}

fn main() -> Result<()> {
    let raw = read_input()?;
    let grid = Grid::parse(&raw)?;
    grid.visible(2, 2);
    let result_a = grid.all().filter(|(x, y)| grid.visible(*x, *y)).count();
    let result_b = grid.max_score();
    println!("Result A: {}\nResult B: {}", result_a, result_b);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_score_a() {
        let input = "555\n555\n555";
        let grid = Grid::parse(input).unwrap();
        assert_eq!(grid.max_score(), 1);
    }
}
