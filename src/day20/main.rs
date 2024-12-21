use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap};
use std::error::Error;
use std::ops::Range;
use std::time::Instant;
use std::{fs, i64};

use aoc24rust::utils::{PointT, RectT};

type Point = PointT<i64>;
type Rect = RectT<i64>;

const WALL: char = '#';
const START: char = 'S';
const END: char = 'E';
const DIRECTIONS: [Point; 4] = [
    Point { x: 0, y: -1 },
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: -1, y: 0 },
];

struct Day20 {
    bounds: Rect,
    level: Vec<Vec<char>>,
    start: Point,
    end: Point,
}

struct PqItem {
    distance: i64,
    position: Point,
    path: Vec<Point>,
}

impl Ord for PqItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        *&other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for PqItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(*&other.distance.cmp(&self.distance))
    }
}

impl PartialEq for PqItem {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for PqItem {}

impl Day20 {
    fn parse(input: &str) -> Option<Day20> {
        let level = input
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let bounds = Rect {
            x: 0,
            y: 0,
            width: level[0].len() as i64,
            height: level.len() as i64,
        };
        let mut start: Option<Point> = None;
        let mut end: Option<Point> = None;
        for (i, line) in level.iter().enumerate() {
            if start.is_none() {
                if let Some(idx) = line.iter().position(|c| *c == START) {
                    start = Some(Point {
                        x: idx as i64,
                        y: i as i64,
                    });
                }
            }
            if end.is_none() {
                if let Some(idx) = line.iter().position(|c| *c == END) {
                    end = Some(Point {
                        x: idx as i64,
                        y: i as i64,
                    });
                }
            }
        }
        if let (Some(start), Some(end)) = (start, end) {
            return Some(Day20 {
                bounds,
                level,
                start,
                end,
            });
        }
        None
    }

    fn get_distances(&self, start: &Point) -> HashMap<Point, i64> {
        let mut distances =
            vec![vec![i64::MAX; self.bounds.width as usize]; self.bounds.height as usize];
        let mut pq: BinaryHeap<PqItem> = BinaryHeap::new();
        pq.push(PqItem {
            distance: 0,
            position: start.clone(),
            path: vec![start.clone()],
        });
        while let Some(item) = pq.pop() {
            if item.distance < distances[item.position.y as usize][item.position.x as usize] {
                distances[item.position.y as usize][item.position.x as usize] = item.distance;
            } else {
                continue;
            }
            for dir in DIRECTIONS {
                let position = dir + item.position.clone();
                let i = PqItem {
                    distance: item.distance + 1,
                    position: position.clone(),
                    path: item
                        .path
                        .clone()
                        .into_iter()
                        .chain([position])
                        .collect::<Vec<Point>>(),
                };
                if self.level[i.position.y as usize][i.position.x as usize] != WALL
                    && i.distance < distances[i.position.y as usize][i.position.x as usize]
                {
                    pq.push(i);
                }
            }
        }
        let mut ret = HashMap::new();
        for (y, row) in distances.iter().enumerate() {
            for (x, dist) in row.iter().enumerate() {
                if *dist < i64::MAX {
                    ret.insert(
                        Point {
                            x: x as i64,
                            y: y as i64,
                        },
                        *dist,
                    );
                }
            }
        }
        return ret;
    }

    fn calculate_cheats(&self, range: Range<i64>) -> HashMap<i64, i64> {
        let start_distances = self.get_distances(&self.start);
        let end_distances = self.get_distances(&self.end);
        let race_distance = start_distances
            .get(&self.end)
            .expect("Maze is not solvable.");
        let mut shortcuts: HashMap<(Point, Point), i64> = HashMap::new();
        for (cheat_start, dist) in start_distances.iter() {
            for cheat_distance in range.clone() {
                for cheat_end in cheat_start.get_points_with_distance(cheat_distance) {
                    match end_distances.get(&cheat_end) {
                        Some(end_dist) => {
                            let shortcut_dist = dist + end_dist + cheat_distance;
                            if shortcut_dist < *race_distance {
                                let mut tmp = vec![cheat_start.clone(), cheat_end];
                                tmp.sort();
                                let key: (Point, Point) = (tmp.remove(0), tmp.remove(0));
                                match shortcuts.entry(key) {
                                    Entry::Occupied(o) => assert!(*o.get() == shortcut_dist),
                                    Entry::Vacant(v) => {
                                        v.insert(*race_distance - shortcut_dist);
                                    }
                                }
                            }
                        }
                        None => continue,
                    }
                }
            }
        }
        let mut ret = HashMap::new();
        for (_, shortcut_dist) in shortcuts {
            *ret.entry(shortcut_dist).or_insert(0) += 1;
        }
        return ret;
    }

    fn part1(&self) -> i64 {
        let cheats = self.calculate_cheats(2..3);
        cheats
            .iter()
            .fold(0_i64, |acc, (k, v)| if *k >= 100 { acc + v } else { acc })
    }

    fn part2(&self) -> i64 {
        let cheats = self.calculate_cheats(2..21);
        cheats
            .iter()
            .fold(0_i64, |acc, (k, v)| if *k >= 100 { acc + v } else { acc })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    match Day20::parse(&fs::read_to_string("src/day20/input.txt")?) {
        Some(solver) => {
            println!("Part1: {}", solver.part1());
            println!("Part2: {}", solver.part2());
        }
        _ => panic!("Invalid input!"),
    };
    let elapsed = start_time.elapsed();
    println!("Run time: {:?}", elapsed);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_example1() {
        let input = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#;
        let solver = Day20::parse(input).expect("Invalid input.");
        let mut shortcuts = solver.calculate_cheats(2..3);
        assert!(shortcuts.remove(&2) == Some(14));
        assert!(shortcuts.remove(&4) == Some(14));
        assert!(shortcuts.remove(&6) == Some(2));
        assert!(shortcuts.remove(&8) == Some(4));
        assert!(shortcuts.remove(&10) == Some(2));
        assert!(shortcuts.remove(&12) == Some(3));
        assert!(shortcuts.remove(&20) == Some(1));
        assert!(shortcuts.remove(&36) == Some(1));
        assert!(shortcuts.remove(&38) == Some(1));
        assert!(shortcuts.remove(&40) == Some(1));
        assert!(shortcuts.remove(&64) == Some(1));
    }
}
