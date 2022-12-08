use anyhow::{anyhow, Error, Result};

fn bad_input(input: &str) -> Error {
    anyhow!("Invalid input: {}", input)
}

struct Section {
    a: i32,
    b: i32,
}

impl Section {
    pub fn parse(input: &str) -> Result<Self> {
        let dash_index = input
            .chars()
            .position(|c| c == '-')
            .ok_or_else(|| bad_input(input))?;

        let a = input[0..dash_index]
            .parse::<i32>()
            .map_err(|_| bad_input(input))?;

        let b = input[dash_index + 1..]
            .parse::<i32>()
            .map_err(|_| bad_input(input))?;

        Ok(Self { a, b })
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.a <= other.a && self.b >= other.b
    }

    pub fn intersect(&self, other: &Self) -> bool {
        (self.a >= other.a && self.a <= other.b) || (self.b >= other.a && self.b <= other.b)
    }
}

struct Pair {
    left: Section,
    right: Section,
}

impl Pair {
    pub fn parse(input: &str) -> Result<Self> {
        let comma_index = input
            .chars()
            .position(|c| c == ',')
            .ok_or_else(|| bad_input(input))?;

        let left = Section::parse(&input[0..comma_index])?;
        let right = Section::parse(&input[comma_index + 1..])?;

        Ok(Self { left, right })
    }

    pub fn overlaps(&self) -> bool {
        self.left.overlaps(&self.right) || self.right.overlaps(&self.left)
    }

    pub fn intersect(&self) -> bool {
        self.left.intersect(&self.right) || self.right.intersect(&self.left)
    }
}

fn main() -> Result<()> {
    let filename = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("missing input filename"))?;

    let data = std::fs::read_to_string(&filename)?;

    let pairs = data
        .lines()
        .map(|line| Pair::parse(line))
        .collect::<Result<Vec<_>>>()?;

    let task_a = pairs.iter().filter(|pair| pair.overlaps()).count();
    println!("Task a: {}", task_a);

    let task_b = pairs.iter().filter(|pair| pair.intersect()).count();
    println!("Task b: {}", task_b);

    Ok(())
}
