use std::collections::{BinaryHeap, HashSet};
use std::error::Error;
use std::fs;
use std::time::Instant;

use aoc24rust::utils::PointT;

type Point = PointT<i64>;

const START: char = 'S';
const END: char = 'E';
const WALL: char = '#';
const DIRECTIONS: [Point; 4] = [
    Point { x: 0, y: 1 },
    Point { x: 1, y: 0 },
    Point { x: 0, y: -1 },
    Point { x: -1, y: 0 },
];
const _PLAYER: [char; 4] = ['^', '>', 'v', '<'];

struct Day16 {
    level: Vec<String>,
    start: Point,
    end: Point,
    direction: i32,
    solution: Option<i64>,
    path: HashSet<Point>,
}

struct QueueItem {
    score: i64,
    pos: Point,
    dir: i32,
    path: HashSet<Point>,
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialEq for QueueItem {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for QueueItem {}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Day16 {
    fn parse(input: &str) -> Option<Day16> {
        let level: Vec<String> = input.lines().map(String::from).collect();
        let mut start_pos: Option<Point> = None;
        let mut end_pos: Option<Point> = None;
        for (y, line) in level.iter().enumerate() {
            if start_pos.is_none() {
                if let Some(idx) = line.find(START) {
                    start_pos = Some(Point {
                        x: idx as i64,
                        y: y as i64,
                    });
                }
            }
            if end_pos.is_none() {
                if let Some(idx) = line.find(END) {
                    end_pos = Some(Point {
                        x: idx as i64,
                        y: y as i64,
                    });
                }
            }
        }
        return match (start_pos, end_pos) {
            (Some(start_pos), Some(end_pos)) => Some(Day16 {
                level: level,
                start: start_pos,
                direction: 1,
                end: end_pos,
                solution: None,
                path: HashSet::new(),
            }),
            _ => None,
        };
    }

    fn add_solution(&mut self, item: &QueueItem) {
        if self.solution.is_none() {
            self.solution = Some(item.score);
            self.path.extend(item.path.clone());
        } else if self.solution == Some(item.score) {
            self.path.extend(item.path.clone());
        }
    }

    fn queue_item(
        &mut self,
        pq: &mut BinaryHeap<QueueItem>,
        visited: &mut HashSet<(Point, i32)>,
        item: QueueItem,
    ) {
        if let Some(sol) = self.solution {
            if item.score > sol {
                return;
            }
        }
        if visited.contains(&(item.pos.clone(), item.dir)) {
            return;
        }
        pq.push(item);
    }

    fn solve(&mut self) {
        let mut pq: BinaryHeap<QueueItem> = BinaryHeap::new();
        let mut visited: HashSet<(Point, i32)> = HashSet::new();
        pq.push(QueueItem {
            score: 0,
            pos: self.start.clone(),
            dir: self.direction,
            path: HashSet::from([self.start.clone()]),
        });

        while let Some(item) = pq.pop() {
            if item.pos == self.end {
                self.add_solution(&item);
                continue;
            } else {
                visited.insert((item.pos.clone(), item.dir));
            }
            // self.draw(Some(&item.pos), Some(&item.dir), Some(&visited));
            // forward
            let target = item.pos.clone() + DIRECTIONS[item.dir as usize].clone();
            match self.level[target.y as usize].chars().nth(target.x as usize) {
                Some(WALL) => {}
                Some(_) => {
                    let mut path_clone = item.path.clone();
                    path_clone.insert(target.clone());
                    self.queue_item(
                        &mut pq,
                        &mut visited,
                        QueueItem {
                            score: item.score + 1,
                            pos: target,
                            dir: item.dir,
                            path: path_clone,
                        },
                    );
                }
                None => todo!(),
            }
            self.queue_item(
                &mut pq,
                &mut visited,
                QueueItem {
                    score: item.score + 1000,
                    pos: item.pos.clone(),
                    dir: (item.dir + 1).rem_euclid(DIRECTIONS.len() as i32),
                    path: item.path.clone(),
                },
            );
            self.queue_item(
                &mut pq,
                &mut visited,
                QueueItem {
                    score: item.score + 1000,
                    pos: item.pos,
                    dir: (item.dir - 1).rem_euclid(DIRECTIONS.len() as i32),
                    path: item.path,
                },
            );
        }
    }

    fn _draw(
        &self,
        pos: Option<&Point>,
        dir: Option<&i32>,
        visited: Option<&HashSet<(Point, i32)>>,
    ) {
        let mut ret = String::new();
        let mut visited_pos: HashSet<Point> = HashSet::new();
        if let Some(visited) = visited {
            visited_pos.extend(visited.iter().map(|pd| pd.0.clone()));
        };
        ret.reserve(self.level.len() * (self.level[0].len() + 2));
        for (y, line) in self.level.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if Some(&Point {
                    x: x as i64,
                    y: y as i64,
                }) == pos
                {
                    match dir {
                        Some(0..4) => ret.push(_PLAYER[*dir.unwrap() as usize]),
                        _ => todo!(),
                    }
                } else if visited_pos.contains(&Point {
                    x: x as i64,
                    y: y as i64,
                }) {
                    ret.push('X');
                } else {
                    ret.push(char);
                }
            }
            ret.push('\n');
        }
        println!("{}", ret);
    }

    fn part1(&self) -> Option<i64> {
        return self.solution;
    }

    fn part2(&self) -> Option<i64> {
        if self.solution.is_some() {
            return Some(self.path.len() as i64);
        }
        None
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();

    match Day16::parse(&fs::read_to_string("src/day16/input.txt")?) {
        Some(mut day) => {
            day.solve();
            println!("Part1: {:?}", day.part1());
            println!("Part2: {:?}", day.part2());
        }
        None => panic!("Invalid input."),
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
        let input = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;
        let day = Day16::parse(input);
        let mut day = day.unwrap();
        day.solve();
        assert!(day.part1() == Some(7036));
        assert!(day.part2() == Some(45));
    }
    #[test]
    fn test_example2() {
        let input = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;
        let day = Day16::parse(input);
        let mut day = day.unwrap();
        day.solve();
        assert!(day.part1() == Some(11048));
        assert!(day.part2() == Some(64));
    }
}
