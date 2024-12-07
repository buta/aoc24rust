use std::collections::HashSet;
use std::collections::{hash_map::Entry, HashMap};
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::time::Instant;

const GUARD: char = '^';
const CLEAR: char = '.';
const OBSTACLE: char = '#';
pub struct Rect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

pub struct Point {
    x: i32,
    y: i32,
}

#[derive(PartialEq, Eq, Hash)]
pub struct Collision {
    x: i32,
    y: i32,
    dir: usize,
}
#[derive(PartialEq, Eq)]
enum SimulationResult {
    LEAVE,
    LOOPING,
}

const DIRECTIONS: [Point; 4] = [
    Point { x: 0, y: -1 },
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: -1, y: 0 },
];

impl Rect {
    fn is_inside(&self, x: i32, y: i32) -> bool {
        return x >= self.x && x < self.width && y >= self.y && y < self.height;
    }
}

struct Simulation<'a> {
    x: i32,
    y: i32,
    bounds: &'a Rect,
    dir: usize,
    data: &'a Vec<char>,
    visited: HashSet<(i32, i32)>,
    collisions: HashMap<Collision, Option<()>>,
}

impl<'a> Simulation<'a> {
    fn new(start_x: i32, start_y: i32, bounds: &'a Rect, data: &'a Vec<char>) -> Simulation<'a> {
        return Simulation {
            x: start_x,
            y: start_y,
            bounds: bounds,
            dir: 0,
            data: data,
            visited: HashSet::new(),
            collisions: HashMap::new(),
        };
    }
    fn turn(&mut self) {
        self.dir = (self.dir + 1) % DIRECTIONS.len();
    }
    fn simulate(&mut self) -> SimulationResult {
        loop {
            self.visited.insert((self.x, self.y));
            let tx = self.x + DIRECTIONS[self.dir].x;
            let ty = self.y + DIRECTIONS[self.dir].y;
            if !self.bounds.is_inside(tx, ty) {
                return SimulationResult::LEAVE;
            }
            match self.data.get((tx + ty * self.bounds.width) as usize) {
                Some(c) => {
                    if *c == OBSTACLE {
                        let coll = Collision {
                            x: tx,
                            y: ty,
                            dir: self.dir,
                        };
                        match self.collisions.entry(coll) {
                            Entry::Occupied(_) => return SimulationResult::LOOPING,
                            Entry::Vacant(v) => v.insert(Some(())),
                        };
                        self.turn();
                        continue;
                    }
                }
                None => panic!("Should not index outside..."),
            }
            self.x = tx;
            self.y = ty;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    let mut data: Vec<char> = Vec::new();
    let mut width: i32 = -1;
    let mut height: i32 = 0;
    let mut start_x: i32 = -1;
    let mut start_y: i32 = -1;
    {
        let file = File::open("src/day06/input.txt")?;
        let reader = BufReader::new(file);
        for line_rs in reader.lines() {
            let line = line_rs?;
            if line.len() as i32 != width {
                if width == -1 {
                    width = line.len() as i32;
                } else {
                    panic!("Map width is inconsistent");
                }
            }
            match line.chars().position(|c| c == GUARD) {
                Some(idx) => {
                    assert!(start_x == -1 && start_y == -1, "Multiple start characters");
                    start_x = idx as i32;
                    start_y = height;
                }
                None => {}
            }
            data.append(&mut line.chars().collect::<Vec<_>>());
            height += 1;
        }
    }

    let bounds = Rect {
        x: 0,
        y: 0,
        width: width,
        height: height,
    };
    let mut solver = Simulation::new(start_x, start_y, &bounds, &data);
    match solver.simulate() {
        SimulationResult::LEAVE => {}
        SimulationResult::LOOPING => panic!("Part1 did not return with LEAVE"),
    }

    let part1 = solver.visited.len();
    println!("Part1: {}", part1);

    let mut part2: i64 = 0;
    for (x, y) in solver.visited.iter() {
        // modify the map
        {
            let chr = data
                .get_mut((x + y * &bounds.width) as usize)
                .expect("Out of bounds");
            if *chr != CLEAR {
                continue;
            }
            *chr = OBSTACLE;
        }
        // solve
        let mut solver2 = Simulation::new(start_x, start_y, &bounds, &data);
        if solver2.simulate() == SimulationResult::LOOPING {
            part2 += 1;
        }
        // undo map modification
        {
            let chr = data
                .get_mut((x + y * &bounds.width) as usize)
                .expect("Out of bounds");
            *chr = CLEAR;
        }
    }

    println!("Part2: {}", part2);
    let elapsed = start_time.elapsed();
    println!("Run time: {:?}", elapsed);

    Ok(())
}
