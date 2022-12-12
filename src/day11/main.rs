use std::fmt::Display;

use anyhow::{anyhow, Result};
use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
enum Number {
    Old,
    Fixed(i64),
}

impl Number {
    pub fn parse(input: &str) -> Result<Self> {
        match input {
            "old" => Ok(Number::Old),
            input => input
                .parse::<i64>()
                .map(|n| Number::Fixed(n))
                .map_err(|_| anyhow!("Invalid number: {input}")),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
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

    fn calculate(&self, value: i64) -> i64 {
        let a = match self.a {
            Number::Old => value,
            Number::Fixed(n) => n,
        };

        let b = match self.b {
            Number::Old => value,
            Number::Fixed(n) => n,
        };

        match self.op {
            Op::Mul => a * b,
            Op::Plus => a + b,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Monkey {
    id: usize,
    items: Vec<i64>,
    operation: Operation,
    test_div: i64,
    on_true: usize,
    on_false: usize,
    inspect_count: usize,
}

impl Monkey {
    pub fn new(
        id: usize,
        items: Vec<i64>,
        operation: Operation,
        test_div: i64,
        on_true: usize,
        on_false: usize,
    ) -> Self {
        Self {
            id,
            items,
            operation,
            test_div,
            on_false,
            on_true,
            inspect_count: 0,
        }
    }

    pub fn process_a(&self, value: i64) -> (i64, usize) {
        let value = self.operation.calculate(value);
        let value = value / 3;
        // let value = value % (23 * 19 * 13 * 17);
        let target = self.target(value);

        (value, target)
    }

    pub fn process_b(&self, value: i64, check: i64) -> (i64, usize) {
        let value = self.operation.calculate(value);
        let value = value % check;
        let target = self.target(value);

        (value, target)
    }

    fn target(&self, value: i64) -> usize {
        if value % self.test_div == 0 {
            self.on_true
        } else {
            self.on_false
        }
    }

    fn take(&mut self) -> Vec<i64> {
        let mut next = vec![];
        std::mem::swap(&mut self.items, &mut next);

        next
    }

    fn add(&mut self, value: i64) {
        self.items.push(value);
    }

    fn inspect(&mut self) {
        self.inspect_count += 1;
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = &self.id;
        let hold = self
            .items
            .iter()
            .map(|item| format!("{item}"))
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "M {id} [{hold}]")
    }
}

#[derive(Clone)]
struct Game {
    monkeys: Vec<Monkey>,
    turn: usize,
    common: i64,
}

impl Game {
    fn new(monkeys: Vec<Monkey>) -> Self {
        let common = monkeys.iter().map(|m| m.test_div).fold(1, |a, e| a * e);
        Self {
            monkeys,
            turn: 0,
            common,
        }
    }

    fn turn_a(&mut self) {
        self.turn += 1;

        for i in 0..self.monkeys.len() {
            let items = self.monkeys[i].take();
            for item in items {
                self.monkeys[i].inspect();
                let (next, target) = self.monkeys[i].process_a(item);
                self.monkeys[target].add(next);
            }
        }
    }
    fn turn_b(&mut self) {
        self.turn += 1;

        for i in 0..self.monkeys.len() {
            let items = self.monkeys[i].take();
            for item in items {
                self.monkeys[i].inspect();
                let (next, target) = self.monkeys[i].process_b(item, self.common);
                self.monkeys[target].add(next);
            }
        }
    }

    fn result(&self) -> usize {
        let mut points = self
            .monkeys
            .iter()
            .map(|m| m.inspect_count)
            .collect::<Vec<_>>();

        points.sort();
        points.reverse();

        points[0] * points[1]
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Turn {}\n", self.turn)?;
        for m in &self.monkeys {
            write!(f, "{m}\n")?;
        }

        Ok(())
    }
}

mod input {
    use super::Monkey;
    use super::Operation;
    use anyhow::{anyhow, Result};
    use regex::Regex;

    pub fn parse_input(input: &str) -> Result<Monkey> {
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

        let monkey = Monkey::new(id, items, op, test_div, on_true, on_false);
        Ok(monkey)
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

    fn parse_starting(input: &str) -> Option<Vec<i64>> {
        lazy_static::lazy_static! {
            static ref RE: Regex = Regex::new(r"Starting items:\s+(.*)").unwrap();
        };

        let items = RE.captures(input)?.get(1)?.as_str();
        items
            .split(", ")
            .map(|i| i.parse::<i64>().ok())
            .collect::<Option<Vec<i64>>>()
    }

    fn parse_test(input: &str) -> Option<i64> {
        lazy_static::lazy_static! {
            static ref RE: Regex = Regex::new(r"Test: divisible by (\d+)").unwrap();
        };

        RE.captures(input)?.get(1)?.as_str().parse::<i64>().ok()
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
    let monkeys = raw
        .split("\n\n")
        .map(parse_input)
        .collect::<Result<Vec<_>>>()?;

    let mut game_a = Game::new(monkeys);
    let mut game_b = game_a.clone();

    for _ in 0..20 {
        game_a.turn_a();
    }

    for _ in 0..10000 {
        game_b.turn_b();
    }

    let result_a = game_a.result();
    let result_b = game_b.result();

    println!("Task A: {result_a}");
    println!("Task B: {result_b}");

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
