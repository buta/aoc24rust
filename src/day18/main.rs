use std::collections::BinaryHeap;
use std::error::Error;
use std::fmt::Debug;
use std::time::Instant;
use std::{fs, i64, vec};

use aoc24rust::utils::{PointT, RectT};

type Point = PointT<i64>;
type Rect = RectT<i64>;

const CLEAR: char = '.';
const CORRUPTED: char = '#';
const DIRECTIONS: [Point; 4] = [
    Point { x: 0, y: -1 },
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: -1, y: 0 },
];

struct PqItem {
    cost: i64,
    point: Point,
}

impl Ord for PqItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        //self.cost.cmp(&other.cost)
        *&other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for PqItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        //Some(self.cost.cmp(&other.cost))
        Some(*&other.cost.cmp(&self.cost))
    }
}

impl PartialEq for PqItem {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for PqItem {}

impl Debug for PqItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{P:{}, C:{}}}", self.point, self.cost)
    }
}

struct Day18 {
    bounds: Rect,
    level: Vec<Vec<char>>,
    falling: Vec<Point>,
    falling_idx: usize,
}

impl Day18 {
    fn parse(bounds: Rect, input: &str) -> Day18 {
        let level = vec![vec![CLEAR; bounds.width as usize]; bounds.height as usize];
        Day18 {
            bounds,
            level,
            falling: input
                .lines()
                .map(|line| {
                    if let [x, y] = line
                        .split(',')
                        .map(|s| s.parse::<i64>())
                        .flatten()
                        .collect::<Vec<i64>>()[..]
                    {
                        Point { x, y }
                    } else {
                        panic!("Invalid input");
                    }
                })
                .collect(),
            falling_idx: 0,
        }
    }

    fn drop(&mut self) {
        let p = &self.falling[self.falling_idx];
        self.falling_idx += 1;
        self.level[p.y as usize][p.x as usize] = CORRUPTED;
    }

    fn solve(&self) -> Option<i64> {
        let mut costs =
            vec![vec![i64::MAX; self.bounds.width as usize]; self.bounds.height as usize];
        let mut pq: BinaryHeap<PqItem> = BinaryHeap::new();
        pq.push(PqItem {
            cost: 0,
            point: Point { x: 0, y: 0 },
        });
        //let mut file = File::create("log.txt").unwrap();
        while let Some(item) = pq.pop() {
            if !self.bounds.contains_point(&item.point) {
                continue;
            }

            if (item.point.x == (self.bounds.width - 1))
                && (item.point.y == (self.bounds.height - 1))
            {
                return Some(item.cost);
            }

            if item.cost < costs[item.point.y as usize][item.point.x as usize] {
                costs[item.point.y as usize][item.point.x as usize] = item.cost;
            } else {
                continue;
            }

            if self.level[item.point.y as usize][item.point.x as usize] == CORRUPTED {
                continue;
            }

            for dir in DIRECTIONS {
                pq.push(PqItem {
                    cost: item.cost + 1,
                    point: item.point.clone() + dir,
                });
            }
        }
        None
    }

    fn get_last_dropped(&self) -> Option<&Point> {
        self.falling.get(self.falling_idx - 1)
    }

    fn _draw(&self, costs: Option<Vec<Vec<i64>>>) {
        let mut tmp: Vec<char> = Vec::new();
        for y in 0..self.bounds.height as usize {
            for x in 0..self.bounds.width as usize {
                let visited = if let Some(costs) = &costs {
                    costs[y][x] != i64::MAX
                } else {
                    false
                };
                if visited {
                    tmp.push('O');
                } else {
                    tmp.push(self.level[y][x]);
                }
            }
            tmp.push('\n');
        }
        println!("{}", tmp.iter().collect::<String>());
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    let bounds = Rect {
        x: 0,
        y: 0,
        width: 71,
        height: 71,
    };
    let mut solver = Day18::parse(bounds, &fs::read_to_string("src/day18/input.txt")?);
    for _ in 0..1024 {
        solver.drop();
    }
    match solver.solve() {
        Some(p1) => println!("Part1: {}", p1),
        None => panic!("Part1 is not solvable?"),
    }
    loop {
        solver.drop();
        if solver.solve().is_none() {
            if let Some(coord) = solver.get_last_dropped() {
                println!("Part2: '{},{}'", coord.x, coord.y);
            } else {
                panic!("Part2 is not solvable?");
            }
            break;
        }
    }
    let elapsed = start_time.elapsed();
    println!("Run time: {:?}", elapsed);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_example1() {
        let input = "5,4\n4,2\n4,5\n3,0\n2,1\n6,3\n2,4\n1,5\n0,6\n3,3\n2,6\n5,1\n1,2\n5,5\n2,5\n6,5\n1,4\n0,4\n6,4\n1,1\n6,1\n1,0\n0,5\n1,6\n2,0";
        let bounds = Rect {
            x: 0,
            y: 0,
            width: 7,
            height: 7,
        };
        let mut solver = Day18::parse(bounds, input);
        for _ in 0..12 {
            solver.drop();
        }
        let result = solver.solve();
        assert!(result == Some(22));
        loop {
            solver.drop();
            if solver.solve().is_none() {
                break;
            }
        }
        assert!(solver.get_last_dropped() == Some(&Point { x: 6, y: 1 }))
    }
}
