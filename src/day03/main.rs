use anyhow::{anyhow, bail, Result};
use std::collections::HashSet;

fn value(input: &char) -> Result<i32> {
    match input {
        'a'..='z' => Ok(*input as i32 - 'a' as i32 + 1),
        'A'..='Z' => Ok(*input as i32 - 'A' as i32 + 27),
        _ => bail!("Invalid common item: {}", input),
    }
}

#[derive(Debug)]
struct Rucksack<'a> {
    items: &'a str,
    size: usize,
}

impl<'a> Rucksack<'a> {
    pub fn parse(items: &'a str) -> Result<Self> {
        if items.len() % 2 != 0 {
            return Err(anyhow!("Invalid lenght for input string: {}", items.len()));
        }

        let size = items.len() / 2;
        Ok(Self { items, size })
    }

    pub fn common(&self) -> Result<char> {
        let left = self.items[0..self.size].chars().collect::<HashSet<_>>();
        let right = self.items[self.size..].chars().collect::<HashSet<_>>();

        let result = left
            .intersection(&right)
            .next()
            .ok_or_else(|| anyhow!("Common char not found"))?;

        Ok(result.clone())
    }

    pub fn items(&self) -> HashSet<char> {
        self.items.chars().collect()
    }
}

#[derive(Debug)]
struct Group<'a> {
    a: &'a Rucksack<'a>,
    b: &'a Rucksack<'a>,
    c: &'a Rucksack<'a>,
}

impl<'a> Group<'a> {
    pub fn new(input: &'a [Rucksack<'a>]) -> Self {
        let a = &input[0];
        let b = &input[1];
        let c = &input[2];

        Self { a, b, c }
    }

    pub fn common(&self) -> Result<char> {
        let ab = self
            .a
            .items()
            .intersection(&self.b.items())
            .cloned()
            .collect::<HashSet<_>>();

        let item = ab
            .intersection(&self.c.items())
            .cloned()
            .next()
            .ok_or_else(|| anyhow!("Common for group not found"))?;

        Ok(item)
    }
}

fn main() -> Result<()> {
    let filename = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("missing input filename"))?;

    let data = std::fs::read_to_string(&filename)?;
    let items = data
        .lines()
        .map(Rucksack::parse)
        .collect::<Result<Vec<_>>>()?;

    let task_a: i32 = items
        .iter()
        .map(|item| item.common().and_then(|c| value(&c)))
        .collect::<Result<Vec<_>>>()?
        .iter()
        .sum();

    println!("Task A: {}", task_a);

    let groups = &items[..]
        .chunks_exact(3)
        .map(Group::new)
        .collect::<Vec<_>>();

    let task_b: i32 = groups
        .iter()
        .map(|item| item.common().and_then(|c| value(&c)))
        .collect::<Result<Vec<_>>>()?
        .iter()
        .sum();

    println!("Task B: {}", task_b);

    Ok(())
}
