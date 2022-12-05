use anyhow::{anyhow, bail, Error, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

#[derive(Clone, Copy)]
struct Crate(char);

impl Crate {
    pub fn new(input: char) -> Self {
        Self(input)
    }
}

impl std::fmt::Debug for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0)
    }
}

struct CrateIter<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> CrateIter<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }
}

impl<'a> Iterator for CrateIter<'a> {
    type Item = Option<Crate>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.input.len() <= self.pos {
            None
        } else {
            let mut i = self.input[self.pos..].chars();
            let l = i.next();
            let c = i.next();
            let r = i.next();

            self.pos += 4;

            match (l, c, r) {
                (Some('['), Some(c), Some(']')) => Some(Some(Crate::new(c))),
                _ => Some(None),
            }
        }
    }
}

struct Stack(Vec<Crate>);

impl Stack {
    pub fn empty() -> Self {
        Self(vec![])
    }

    pub(crate) fn add(&mut self, c: Crate) {
        let mut next = vec![c];
        next.append(&mut self.0);

        self.0 = next;
    }
}

impl std::fmt::Debug for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.0.iter() {
            write!(f, "{:?} ", c)?;
        }

        Ok(())
    }
}

struct Field(HashMap<usize, Stack>);

impl std::fmt::Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut keys = self.0.keys().cloned().collect::<Vec<_>>();
        keys.sort();

        for k in keys {
            let stack = self.0.get(&k).unwrap();
            write!(f, "\n{}: {:?}", k, stack)?;
        }

        Ok(())
    }
}

impl Field {
    pub fn parse(input: &str) -> Self {
        let mut stacks = HashMap::new();

        for line in input.lines() {
            for (idx, c) in CrateIter::new(line).enumerate() {
                let number = idx + 1;

                let stack = stacks.entry(number).or_insert(Stack::empty());

                if let Some(c) = c {
                    stack.add(c);
                }
            }
        }

        Self(stacks)
    }

    pub fn process(&mut self, m: &Move) {
        for _ in 0..m.count {
            let from = self.0.get_mut(&m.from).unwrap();
            let c = from.0.pop().unwrap();
            let to = self.0.get_mut(&m.to).unwrap();
            to.0.push(c);
        }
    }

    pub fn result(&self) -> String {
        let mut keys = self.0.keys().cloned().collect::<Vec<_>>();
        keys.sort();

        keys.iter()
            .map(|c| self.0.get(c).unwrap().0.last().unwrap().0)
            .collect()
    }
}

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl std::fmt::Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {} [{}]", self.from, self.to, self.count)
    }
}

impl Move {
    pub fn parse(input: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        }

        let matches = RE
            .captures(input)
            .ok_or_else(|| anyhow!("invalid move: {}", input))?;

        let count = matches
            .get(1)
            .and_then(|v| v.as_str().parse::<usize>().ok())
            .ok_or_else(|| anyhow!("invalid move: {}", input))?;

        let from = matches
            .get(2)
            .and_then(|v| v.as_str().parse::<usize>().ok())
            .ok_or_else(|| anyhow!("invalid move: {}", input))?;

        let to = matches
            .get(3)
            .and_then(|v| v.as_str().parse::<usize>().ok())
            .ok_or_else(|| anyhow!("invalid move: {}", input))?;

        Ok(Self { count, from, to })
    }
}

#[derive(Debug)]
struct Game {
    field: Field,
    moves: Vec<Move>,
}

impl Game {
    pub fn parse(input: &str) -> Result<Self> {
        let mut parts = input.split("\n\n");
        let field_input = parts.next().ok_or_else(|| anyhow!("missing first part!"))?;
        let field = Field::parse(field_input);

        let moves_input = parts.next().ok_or_else(|| anyhow!("missing first part!"))?;

        let moves = moves_input
            .lines()
            .map(Move::parse)
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { field, moves })
    }

    pub fn process(&mut self) {
        for m in &self.moves {
            self.field.process(m);
        }
    }

    pub fn result(&self) -> String {
        self.field.result()
    }
}

fn main() -> Result<()> {
    let filename = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("missing input filename"))?;

    let data = std::fs::read_to_string(&filename)?;
    let mut game = Game::parse(&data)?;
    dbg!(&game);

    game.process();
    dbg!(&game);

    println!("{}", game.result());

    Ok(())
}
