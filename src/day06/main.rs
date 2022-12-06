use anyhow::{anyhow, Result};
use std::collections::HashSet;

struct Game<'a> {
    input: &'a str,
}

impl<'a> Game<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }

    pub fn result(&self, size: usize) -> Option<usize> {
        let limit = self.input.len() - (size - 1);

        for n in 0..limit {
            let part = &self.input[n..n + size];
            let set: HashSet<_> = part.chars().collect();
            if set.len() == size {
                return Some(n + size);
            }
        }

        None
    }
}

pub fn main() -> Result<()> {
    let filename = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("missing input filename"))?;

    let data = std::fs::read_to_string(&filename)?;
    let game = Game::new(&data);

    let task_a = game.result(4);
    let task_b = game.result(14);

    match task_a {
        Some(n) => println!("Task A: {}", n),
        _ => println!("Task A: non found"),
    }

    match task_b {
        Some(n) => println!("Task B: {}", n),
        _ => println!("Task B: non found"),
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let result_a = Game::new(input).result(4);
        let result_b = Game::new(input).result(14);

        assert_eq!(result_a, Some(7));
        assert_eq!(result_b, Some(19));
    }
}
