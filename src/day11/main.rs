use anyhow::{anyhow, bail, Result};
use regex::Regex;

#[derive(Debug, PartialEq)]
enum Number {
    Old,
    Fixed(i32),
}

impl Number {
    pub fn parse(input: &str) -> Result<Self> {
        match input {
            "old" => Ok(Number::Old),
            input => input
                .parse::<i32>()
                .map(|n| Number::Fixed(n))
                .map_err(|_| anyhow!("Invalid number: {input}")),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Op {
    Plus,
    Mul,
}

impl Op {
    pub fn parse(input: &str) -> Result<Self> {
        match input {
            "*" => Ok(Op::Mul),
            "+" => Ok(Op::Plus),
            _ => Err(anyhow!("Invalid op: {input}")),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Operation {
    a: Number,
    op: Op,
    b: Number,
}

impl Operation {
    pub fn parse(input: &str) -> Result<Self> {
        lazy_static::lazy_static! {
            static ref RE: Regex = Regex::new(r"new = (\w+) ([*+]) (\w+)").unwrap();
        };

        let err = || anyhow!("Invalid input: {input}");
        let caps = RE.captures(input).ok_or_else(err)?;
        let a = caps
            .get(1)
            .ok_or_else(err)
            .and_then(|v| Number::parse(v.as_str()))?;

        let op = caps
            .get(2)
            .ok_or_else(err)
            .and_then(|v| Op::parse(v.as_str()))?;

        let b = caps
            .get(3)
            .ok_or_else(err)
            .and_then(|v| Number::parse(v.as_str()))?;

        Ok(Self { a, op, b })
    }
}

#[derive(Debug)]
pub struct Monkey {
    id: usize,
    operation: Operation,
    test_div: i32,
    on_true: usize,
    on_false: usize,
}

impl Monkey {
    pub fn new(
        id: usize,
        operation: Operation,
        test_div: i32,
        on_false: usize,
        on_true: usize,
    ) -> Self {
        Self {
            id,
            operation,
            test_div,
            on_false,
            on_true,
        }
    }
}

mod input {
    use super::Monkey;
    use super::Operation;
    use anyhow::{anyhow, Result};
    use regex::Regex;

    pub fn parse_input(input: &str) -> Result<(Monkey, Vec<i32>)> {
        let err = |key: &str| anyhow!("Err: {}", key);
        let mut lines = input.lines();
        let id = lines
            .next()
            .and_then(parse_id)
            .ok_or_else(|| err("ID error"))?;

        let items = lines
            .next()
            .and_then(parse_starting)
            .ok_or_else(|| err("Starting err"))?;

        let op = lines.next().and_then(parse_op).ok_or_else(|| err("op"))?;

        let test_div = lines
            .next()
            .and_then(parse_test)
            .ok_or_else(|| err("test"))?;

        let on_true = lines
            .next()
            .and_then(parse_throw)
            .ok_or_else(|| err("on true"))?;

        let on_false = lines
            .next()
            .and_then(parse_throw)
            .ok_or_else(|| err("on false"))?;

        let monkey = Monkey::new(id, op, test_div, on_true, on_false);

        Ok((monkey, items))
    }

    fn parse_op(input: &str) -> Option<Operation> {
        lazy_static::lazy_static! {
            static ref RE: Regex = Regex::new(r"Operation: (.*)").unwrap();
        };
        let op_str = RE.captures(input)?.get(1)?.as_str();
        Operation::parse(op_str).ok()
    }

    fn parse_id(input: &str) -> Option<usize> {
        lazy_static::lazy_static! {
            static ref RE: Regex = Regex::new(r"Monkey (\d+)").unwrap();
        };

        RE.captures(input)?.get(1)?.as_str().parse::<usize>().ok()
    }

    fn parse_starting(input: &str) -> Option<Vec<i32>> {
        lazy_static::lazy_static! {
            static ref RE: Regex = Regex::new(r"Starting items:\s+(.*)").unwrap();
        };

        let items = RE.captures(input)?.get(1)?.as_str();
        items
            .split(", ")
            .map(|i| i.parse::<i32>().ok())
            .collect::<Option<Vec<i32>>>()
    }

    fn parse_test(input: &str) -> Option<i32> {
        lazy_static::lazy_static! {
            static ref RE: Regex = Regex::new(r"Test: divisible by (\d+)").unwrap();
        };

        RE.captures(input)?.get(1)?.as_str().parse::<i32>().ok()
    }

    fn parse_throw(input: &str) -> Option<usize> {
        lazy_static::lazy_static! {
            static ref RE: Regex = Regex::new(r"If (true|false): throw to monkey (\d+)").unwrap();
        };

        RE.captures(input)?.get(2)?.as_str().parse::<usize>().ok()
    }
}

use input::parse_input;

fn main() -> Result<()> {
    let raw = advent2022::read_input()?;
    let blocks = raw
        .split("\n\n")
        .map(parse_input)
        .collect::<Result<Vec<_>>>()?;

    dbg!(blocks);

    println!("Test");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_op1() {
        let input = "new = old * 19";
        let op = Operation::parse(input).unwrap();
        let expected = Operation {
            a: Number::Old,
            op: Op::Mul,
            b: Number::Fixed(19),
        };

        assert_eq!(op, expected);
    }
}
