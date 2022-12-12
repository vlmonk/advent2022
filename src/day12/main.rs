use std::collections::{HashMap, HashSet};

use anyhow::Result;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn arond(&self) -> impl Iterator<Item = Self> {
        vec![
            Point::new(self.x - 1, self.y),
            Point::new(self.x, self.y + 1),
            Point::new(self.x + 1, self.y),
            Point::new(self.x, self.y - 1),
        ]
        .into_iter()
    }
}

#[derive(Debug)]
struct HeightMap {
    points: HashMap<Point, i32>,
}

impl HeightMap {
    pub fn new() -> Self {
        Self {
            points: HashMap::new(),
        }
    }

    pub fn add(&mut self, point: Point, heihth: i32) {
        self.points.insert(point, heihth);
    }

    pub fn get(&self, point: &Point) -> Option<i32> {
        self.points.get(point).cloned()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Point, &i32)> {
        self.points.iter()
    }
}

#[derive(Debug)]

struct Input {
    height_map: HeightMap,
    start_point: Point,
    target: Point,
}

impl Input {
    pub fn parse(input: &str) -> Self {
        let mut start_point: Option<Point> = None;
        let mut target: Option<Point> = None;
        let mut height_map = HeightMap::new();

        for (y, line) in input.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let point = Point::new(x as i32, y as i32);
                match ch {
                    'S' => {
                        height_map.add(point.clone(), Input::height('a'));
                        start_point = Some(point);
                    }
                    'E' => {
                        height_map.add(point.clone(), Input::height('z'));
                        target = Some(point)
                    }
                    'a'..='z' => height_map.add(point, Input::height(ch)),
                    _ => panic!("invalid height: {ch}"),
                }
            }
        }

        let start_point = start_point.expect("Start point not found");
        let target = target.expect("Target not found");

        Self {
            height_map,
            start_point,
            target,
        }
    }

    fn height(ch: char) -> i32 {
        ch as i32 - 'a' as i32
    }
}

#[derive(Debug)]
struct Game<'a> {
    height_map: &'a HeightMap,
    target: Point,
    visited: HashMap<Point, usize>,
    queue: HashSet<Point>,
    step: usize,
}

impl<'a> Game<'a> {
    pub fn new(start_point: Point, target: Point, height_map: &'a HeightMap) -> Self {
        let mut visited = HashMap::new();
        visited.insert(start_point.clone(), 0);

        let mut queue = HashSet::new();
        queue.insert(start_point);

        let step = 0;

        Self {
            height_map,
            target,
            visited,
            queue,
            step,
        }
    }

    fn slove(&mut self) -> Option<usize> {
        loop {
            if let Some(step) = self.visited.get(&self.target) {
                return Some(*step);
            }

            if self.queue.is_empty() {
                return None;
            }

            self.step()
        }
    }

    fn step(&mut self) {
        self.step += 1;
        let mut next = HashSet::new();
        for q in self.queue.drain() {
            let current = self.height_map.get(&q).expect("Invalid point in queue");

            for p in q.arond() {
                let height = self.height_map.get(&p);
                if let Some(height) = height {
                    if current + 1 >= height {
                        if !self.visited.contains_key(&p) {
                            self.visited.insert(p.clone(), self.step);
                            next.insert(p);
                        }
                    }
                }
            }
        }

        self.queue = next;
    }
}

fn main() -> Result<()> {
    let raw = advent2022::read_input()?;
    let input = Input::parse(&raw);

    let mut game = Game::new(
        input.start_point.clone(),
        input.target.clone(),
        &input.height_map,
    );

    let result_a = game.slove().expect("Task a Not solved");
    println!("Result A: {result_a}");

    let start_points = input.height_map.iter().filter_map(|(point, height)| {
        if height == &0 {
            Some(point.clone())
        } else {
            None
        }
    });

    let result_b = start_points
        .filter_map(|p| {
            let mut game = Game::new(p, input.target.clone(), &input.height_map);
            game.slove()
        })
        .min()
        .expect("Task b Not solved");

    println!("Result B: {result_b}");

    Ok(())
}
