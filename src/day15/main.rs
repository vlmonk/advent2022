use std::collections::VecDeque;

use advent2022::read_input;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Sensor {
    sx: i64,
    sy: i64,
    bx: i64,
    by: i64,
}

impl Sensor {
    pub fn parse(input: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"
            )
            .unwrap();
        }

        let caps = RE.captures(input).unwrap();
        let sx = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let sy = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let bx = caps.get(3).unwrap().as_str().parse::<i64>().unwrap();
        let by = caps.get(4).unwrap().as_str().parse::<i64>().unwrap();

        Self { sx, sy, bx, by }
    }

    pub fn radius(&self) -> i64 {
        (self.sx - self.bx).abs() + (self.sy - self.by).abs()
    }

    pub fn at(&self, y: i64) -> Option<Segment> {
        let dy = (self.sy - y).abs();
        let radius = self.radius() - dy;

        if radius >= 0 {
            Some(Segment {
                a: self.sx - radius,
                b: self.sx + radius,
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
struct Segment {
    a: i64,
    b: i64,
}

impl Segment {
    pub fn len(&self) -> i64 {
        self.b - self.a
    }
}

#[derive(Debug)]
struct SegmentSet {
    segments: VecDeque<Segment>,
}

impl SegmentSet {
    pub fn new() -> Self {
        Self {
            segments: VecDeque::new(),
        }
    }

    pub fn add(&mut self, segment: &Segment) {
        if let Some(last) = self.segments.back_mut() {
            if segment.a > last.b + 1 {
                self.segments.push_back(segment.clone());
            } else {
                last.b = last.b.max(segment.b);
            }
        } else {
            self.segments.push_back(segment.clone())
        }
    }

    pub fn len(&self) -> i64 {
        self.segments.iter().map(|s| s.len()).sum()
    }

    pub fn count(&self) -> usize {
        self.segments.len()
    }

    pub fn hole(&self) -> Option<i64> {
        if self.segments.len() == 2 {
            Some(self.segments[0].b + 1)
        } else {
            None
        }
    }
}

struct Field {
    sensors: Vec<Sensor>,
}

impl Field {
    fn new(sensors: Vec<Sensor>) -> Self {
        Self { sensors }
    }

    fn at(&self, target: i64) -> SegmentSet {
        let mut segments: Vec<_> = self.sensors.iter().filter_map(|s| s.at(target)).collect();
        segments.sort_by(|a, b| a.a.cmp(&b.a));

        let mut set = SegmentSet::new();

        for s in &segments {
            set.add(s)
        }

        set
    }

    fn hole_at(&self, target: i64) -> Option<(i64, i64)> {
        let set = self.at(target);
        if let Some(x) = set.hole() {
            Some((x, target))
        } else {
            None
        }
    }
}

const TARGET_A: i64 = 2000000;
const TARGET_B: i64 = 4000000;

fn main() {
    let raw = read_input().unwrap();
    let sensors: Vec<_> = raw.lines().map(Sensor::parse).collect();
    let field = Field::new(sensors);

    let task_a = field.at(TARGET_A).len();
    println!("Task A: {task_a}");

    let task_b = (0..=TARGET_B).find_map(|y| field.hole_at(y));
    if let Some((x, y)) = task_b {
        let result = x * 4000000 + y;
        println!("Task B: {result}");
    }
}
