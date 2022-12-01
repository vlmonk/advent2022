#[derive(Debug)]
struct Elf {
    food: Vec<i64>,
}

impl Elf {
    fn parse(input: &str) -> Result<Self> {
        let food = input
            .split("\n")
            .filter(|line| line.len() > 0)
            .map(|line| {
                line.parse::<i64>()
                    .map_err(|_| anyhow!("Invalid input: {}", line))
            })
            .collect::<Result<Vec<_>>>()?;

        let elf = Self { food };
        Ok(elf)
    }

    fn total(&self) -> i64 {
        self.food.iter().sum()
    }
}

use anyhow::anyhow;
use anyhow::Result;

fn main() -> Result<()> {
    let filename = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("missing input filename"))?;

    let data = std::fs::read_to_string(&filename)?;

    let elfs = data
        .split("\n\n")
        .map(|input| Elf::parse(input))
        .collect::<Result<Vec<_>>>()?;

    let mut foods = elfs.iter().map(|e| e.total()).collect::<Vec<_>>();

    foods.sort();
    foods.reverse();

    let max = foods.iter().nth(0);
    dbg!(max);

    let max_3: i64 = foods.iter().take(3).sum();
    dbg!(max_3);

    Ok(())
}
