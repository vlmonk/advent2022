use std::collections::HashMap;

use anyhow::Result;

#[derive(Debug)]
enum Segment {
    Horizontal { xa: i32, xb: i32, y: i32 },
    Vertical { x: i32, ya: i32, yb: i32 },
}

impl Segment {
    pub fn from_points(a: &Point, b: &Point) -> Self {
        if a.y == b.y {
            let xmin = a.x.min(b.x);
            let xmax = a.x.max(b.x);

            Self::Horizontal {
                xa: xmin,
                xb: xmax,
                y: a.y,
            }
        } else if a.x == b.x {
            let ymin = a.y.min(b.y);
            let ymax = a.y.max(b.y);

            Self::Vertical {
                x: a.x,
                ya: ymin,
                yb: ymax,
            }
        } else {
            panic!("Invalid points: {:?} {:?}", a, b);
        }
    }

    fn points(&self) -> impl Iterator<Item = Point> + '_ {
        let points: Vec<Point> = match self {
            Segment::Horizontal { xa, xb, y } => (*xa..=*xb).map(|x| Point::new(x, *y)).collect(),
            Segment::Vertical { x, ya, yb } => (*ya..=*yb).map(|y| Point::new(*x, y)).collect(),
        };

        points.into_iter()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn parse(input: &str) -> Self {
        let mut parts = input.split(',');
        let x = parts
            .next()
            .and_then(|s| s.parse::<i32>().ok())
            .expect("can't parse X");

        let y = parts
            .next()
            .and_then(|s| s.parse::<i32>().ok())
            .expect("Can't parse Y");

        Self { x, y }
    }

    pub fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn down_left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    pub fn down_right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
}

fn parse_segments(input: &str) -> Vec<Segment> {
    let parts = input.split(" -> ").map(Point::parse).collect::<Vec<_>>();
    parts
        .windows(2)
        .map(|w| Segment::from_points(&w[0], &w[1]))
        .collect::<Vec<_>>()
}

#[derive(Debug)]
struct BoundBox {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl BoundBox {
    pub fn new(point: &Point) -> Self {
        Self {
            x_min: point.x,
            x_max: point.x,
            y_min: point.y,
            y_max: point.y,
        }
    }

    fn add_segment(&mut self, segment: &Segment) {
        match segment {
            Segment::Horizontal { xa, xb, y } => {
                self.x_min = self.x_min.min(*xa);
                self.x_max = self.x_max.max(*xb);
                self.y_min = self.y_min.min(*y);
                self.y_max = self.y_max.max(*y);
            }
            Segment::Vertical { x, ya, yb } => {
                self.x_min = self.x_min.min(*x);
                self.x_max = self.x_max.max(*x);
                self.y_min = self.y_min.min(*ya);
                self.y_max = self.y_max.max(*yb);
            }
        }
    }
}

#[derive(Debug)]
struct Grid {
    start_point: Point,
    bbox: BoundBox,
    cells: HashMap<Point, Cell>,
    has_floor: bool,
}

impl Grid {
    fn new(point: &Point, has_floor: bool) -> Self {
        let bbox = BoundBox::new(&point);
        let cells = HashMap::new();

        Self {
            cells,
            start_point: point.clone(),
            bbox,
            has_floor,
        }
    }

    fn add_segment(&mut self, segment: &Segment) {
        self.bbox.add_segment(segment);
        for point in segment.points() {
            self.add_solid(point);
        }
    }

    fn add_solid(&mut self, point: Point) {
        self.cells.insert(point, Cell::Solid);
    }

    fn add_sand(&mut self, point: Point) {
        self.cells.insert(point, Cell::Sand);
    }

    fn drop_sand(&mut self) -> DropResult {
        let mut sand = self.start_point.clone();

        if !self.is_empty(&sand) {
            return DropResult::Blocked;
        }

        loop {
            if sand.y > self.bbox.y_max + 3 {
                return DropResult::Fall;
            }

            if self.is_empty(&sand.down()) {
                sand = sand.down()
            } else if self.is_empty(&sand.down_left()) {
                sand = sand.down_left()
            } else if self.is_empty(&sand.down_right()) {
                sand = sand.down_right()
            } else {
                self.add_sand(sand);
                return DropResult::Stay;
            }
        }
    }

    fn is_empty(&self, p: &Point) -> bool {
        if self.has_floor && p.y >= self.bbox.y_max + 2 {
            return false;
        }

        !self.cells.contains_key(p)
    }
}

#[derive(Debug)]
enum Cell {
    Solid,
    Sand,
}

#[derive(Debug, PartialEq)]
enum DropResult {
    Stay,
    Fall,
    Blocked,
}

fn main() -> Result<()> {
    let raw = advent2022::read_input()?;
    let segments = raw
        .lines()
        .map(parse_segments)
        .flatten()
        .collect::<Vec<_>>();

    let start_point = Point::new(500, 0);

    let mut grid_a = Grid::new(&start_point, false);
    let mut grid_b = Grid::new(&start_point, true);

    for segment in &segments {
        grid_a.add_segment(segment);
        grid_b.add_segment(segment);
    }

    let result_a = std::iter::repeat(())
        .map(|_| grid_a.drop_sand())
        .take_while(|r| *r == DropResult::Stay)
        .count();

    let result_b = std::iter::repeat(())
        .map(|_| grid_b.drop_sand())
        .take_while(|r| *r != DropResult::Blocked)
        .count();

    println!("Task A: {result_a}");
    println!("Task B: {result_b}");

    Ok(())
}
