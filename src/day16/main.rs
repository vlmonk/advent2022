use std::collections::HashMap;

use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Room {
    name: String,
    rate: i32,
    dst: Vec<String>,
}

impl Room {
    pub fn parse(input: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.*)")
                    .unwrap();
        }

        let err = || anyhow::anyhow!("Can't parse input: {}", input);

        let caps = RE.captures(input).ok_or_else(err)?;
        let name = caps.get(1).map(|v| v.as_str().to_owned()).ok_or_else(err)?;
        let rate = caps
            .get(2)
            .and_then(|v| v.as_str().parse::<i32>().ok())
            .ok_or_else(err)?;

        let dst = caps
            .get(3)
            .map(|v| {
                v.as_str()
                    .split(", ")
                    .map(|chunk| chunk.to_owned())
                    .collect::<Vec<_>>()
            })
            .ok_or_else(err)?;

        Ok(Self { name, rate, dst })
    }
}

#[derive(Debug)]
struct Map {
    rooms: Vec<Room>,
    valves: usize,
    total: i32,
}

impl Map {
    pub fn parse(input: &str) -> Result<Self> {
        let rooms = input.lines().map(Room::parse).collect::<Result<Vec<_>>>()?;
        let valves = rooms.iter().filter(|r| r.rate > 0).count();
        let total = rooms.iter().map(|r| r.rate).sum();

        Ok(Self {
            rooms,
            valves,
            total,
        })
    }

    pub fn find(&self, name: &str) -> &Room {
        let founed = self.rooms.iter().find(|el| el.name == name);
        founed.unwrap()
    }
}

#[derive(Debug, Clone, PartialEq)]
struct State {
    current: String,
    open: Vec<String>,
    per_min: i32,
    total: i32,
}

impl State {
    fn new(current: &str) -> Self {
        let current = current.to_owned();
        let open = vec![];

        Self {
            current,
            open,
            per_min: 0,
            total: 0,
        }
    }

    fn process(&self, action: &Action, map: &Map) -> Self {
        let room = map.find(&self.current);
        let mut state = self.clone();
        state.total += state.per_min;

        match action {
            Action::Open => {
                state.per_min += room.rate;
                state.open.push(self.current.clone());
                state.open.sort();
            }
            Action::MoveTo(to) => state.current = to.clone(),
            Action::Sleep => {}
        };

        state
    }
}

struct Game {
    map: Map,
    states: Vec<State>,
    remnant: usize,
}

impl Game {
    fn new(map: Map, steps: usize) -> Self {
        let states = vec![State::new("AA")];
        Self {
            map,
            states,
            remnant: steps,
        }
    }

    fn tick(&mut self) {
        self.remnant -= 1;

        type Key = (String, Vec<String>);
        let mut states: HashMap<Key, State> = HashMap::new();

        for s in &self.states {
            for action in self.actions(s, &self.map) {
                let next = s.process(&action, &self.map);
                let total = next.total;
                let key: Key = (next.current.clone(), next.open.clone());
                let entry = states.entry(key).or_insert(next);
                if total > entry.total {
                    entry.total = total
                }
            }
        }

        let states: Vec<_> = states.drain().map(|(_, v)| v).collect();

        let actual_max = self
            .states
            .iter()
            .map(|s| s.total + s.per_min * self.remnant as i32)
            .max()
            .unwrap();

        let states = states
            .into_iter()
            .filter(|s| {
                let poss = s.total + self.map.total * self.remnant as i32;
                poss >= actual_max
            })
            .collect();

        self.states = states;
    }

    fn actions(&self, state: &State, map: &Map) -> Vec<Action> {
        let room = self.map.find(&state.current);
        let mut actions = vec![];

        if state.open.len() == map.valves {
            actions.push(Action::Sleep);
            return actions;
        }

        for dst in &room.dst {
            actions.push(Action::MoveTo(dst.clone()))
        }

        if room.rate > 0 && !state.open.contains(&state.current) {
            actions.push(Action::Open);
        }

        actions
    }
}

enum Action {
    Open,
    MoveTo(String),
    Sleep,
}

fn main() -> Result<()> {
    let input = advent2022::read_input()?;
    let map = Map::parse(&input)?;
    let mut game = Game::new(map, 30);
    for n in 1..=30 {
        game.tick();
        println!("Tick {} - {} - {}", n, game.states.len(), game.remnant);
    }

    let a = game.states.iter().map(|s| s.total).max();
    dbg!(a);
    Ok(())
}
