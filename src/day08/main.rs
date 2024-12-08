use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::time::Instant;

const CLEAR: char = '.';
pub struct Rect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Rect {
    fn is_inside(&self, p: &Point) -> bool {
        return p.x >= self.x && p.x < self.width && p.y >= self.y && p.y < self.height;
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn sub(a: i32, b: i32) -> i32 {
    return a - b;
}

const OPERATIONS: [fn(i32, i32) -> i32; 2] = [add, sub];

fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    let mut frequencies: HashMap<char, Vec<Point>> = HashMap::new();
    // read and validate input file
    let file = File::open("src/day08/input.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<_> = reader
        .lines()
        .into_iter()
        .map(|v| v.expect("Line is missing"))
        .collect();
    assert!(
        lines
            .iter()
            .all(|v| v.len() == lines.first().expect("Empty input").len()),
        "Map width is not uniform"
    );
    let width = lines.first().expect("Empty input").len();
    let bounds = Rect {
        x: 0,
        y: 0,
        width: width as i32,
        height: lines.len() as i32,
    };
    // extract points
    for (y, line_) in lines.iter().enumerate() {
        let y = y as i32;
        for (x, c) in line_.chars().enumerate() {
            if c != CLEAR {
                let x = x as i32;
                let p = Point { x, y };
                match frequencies.entry(c) {
                    Entry::Occupied(mut o) => o.get_mut().push(p),
                    Entry::Vacant(v) => {
                        v.insert(vec![p]);
                    }
                }
            }
        }
    }
    // find the antinodes
    let mut antinodes_part1: HashSet<Point> = HashSet::new();
    let mut antinodes_part2: HashSet<Point> = HashSet::new();
    for nodes in frequencies.values() {
        for a in nodes.iter() {
            for b in nodes.iter() {
                if a == b {
                    continue;
                }
                antinodes_part2.insert(a.clone());
                antinodes_part2.insert(b.clone());
                let dx = a.x - b.x;
                let dy = a.y - b.y;

                for (op, item) in OPERATIONS.iter().zip([a, b]) {
                    for i in 1.. {
                        let p = Point {
                            x: op(item.x, dx * i),
                            y: op(item.y, dy * i),
                        };
                        if bounds.is_inside(&p) {
                            if i == 1 {
                                antinodes_part1.insert(p.clone());
                            }
                            antinodes_part2.insert(p);
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    let part1 = antinodes_part1.len();
    let part2 = antinodes_part2.len();

    let elapsed = start_time.elapsed();
    println!("Part1: {}\nPart2: {}", part1, part2);
    println!("Run time: {:?}", elapsed);
    Ok(())
}
