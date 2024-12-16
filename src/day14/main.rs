use std::{collections::HashSet, error::Error, fmt::Debug, fs, time::Instant};

use aoc24rust::utils::{PointT, RectT};

type Point = PointT<i64>;
type Rect = RectT<i64>;

struct Robot<'a> {
    position: Point,
    velocity: Point,
    bounds: &'a Rect,
}

struct Day<'a> {
    bounds: &'a Rect,
    robots: Vec<Robot<'a>>,
}

const DIRECTIONS: [Point; 4] = [
    Point { x: 0, y: 1 },
    Point { x: 1, y: 0 },
    Point { x: 0, y: -1 },
    Point { x: -1, y: 0 },
];

impl<'a> Debug for Robot<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Position:{}, Velocity:{}", self.position, self.velocity)
    }
}
impl<'a> Robot<'a> {
    fn parse(line: &str, bounds: &'a Rect) -> Option<Robot<'a>> {
        let parse_point = |part: &str| {
            if let [x, y] = part
                .split(",")
                .collect::<Vec<&str>>()
                .into_iter()
                .filter_map(|s| -> Option<i64> {
                    match s.chars().rev().position(|c| !(c.is_digit(10) || c == '-')) {
                        Some(idx) => &s[s.len() - idx..],
                        None => s,
                    }
                    .parse::<i64>()
                    .ok()
                })
                .collect::<Vec<i64>>()[..]
            {
                return Some(Point { x, y });
            }
            None
        };
        if let [posstr, velstr] = line.split(" ").collect::<Vec<&str>>()[..] {
            let pos = parse_point(posstr);
            let vel = parse_point(velstr);
            match (pos, vel) {
                (Some(pos), Some(vel)) => {
                    return Some(Robot {
                        position: pos,
                        velocity: vel,
                        bounds: bounds,
                    })
                }
                _ => {}
            };
        }
        None
    }

    fn get_position(&self, seconds: i64) -> Point {
        let pos = self.position.clone() + (self.velocity.clone() * seconds);
        return Point {
            x: pos.x.rem_euclid(self.bounds.width),
            y: pos.y.rem_euclid(self.bounds.height),
        };
    }
}

impl<'a> Day<'a> {
    fn parse(input: &str, bounds: &'a Rect) -> Option<Day<'a>> {
        Some(Day {
            bounds: bounds,
            robots: input
                .lines()
                .into_iter()
                .map(|line| Robot::parse(line, bounds))
                .flatten()
                .collect::<Vec<Robot>>(),
        })
    }
    fn part1(&self) -> i64 {
        let positions = self
            .robots
            .iter()
            .map(|r| r.get_position(100))
            .collect::<Vec<Point>>();
        let solution = self
            .bounds
            .get_quadrants()
            .iter()
            .map(|q| positions.iter().filter(|p| q.contains_point(p)).count())
            .fold(1, |sum, v| sum * v);
        solution as i64
    }
    fn part2(&self) -> i64 {
        // half of the robots should have at least two neighbours... ...probably
        for i in 1..100000_i64 {
            let positions = self
                .robots
                .iter()
                .map(|r| r.get_position(i))
                .collect::<HashSet<Point>>();
            fn get_neigbours(p: &Point) -> [Point; 4] {
                return DIRECTIONS.map(|dir| p.clone() + dir);
            }
            let two_neighbours = positions
                .iter()
                .filter(|p| {
                    get_neigbours(p)
                        .iter()
                        .filter(|n| positions.contains(n))
                        .count()
                        >= 2
                })
                .count();

            if two_neighbours > positions.len() / 2 {
                return i;
            }
        }
        -1
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    let bounds = Rect {
        x: 0,
        y: 0,
        width: 101,
        height: 103,
    };
    match Day::parse(&fs::read_to_string("src/day14/input.txt")?, &bounds) {
        Some(day) => {
            println!("Part1: {}", day.part1());
            println!("Part2: {}", day.part2());
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
    fn test_parse() {
        let input = r#"p=56,68 v=1,-12"#;
        let bounds = Rect {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        let robot = Robot::parse(input, &bounds);
        match robot {
            Some(robot) => {
                assert!(robot.position == Point { x: 56, y: 68 });
                assert!(robot.velocity == Point { x: 1, y: -12 });
            }
            None => assert!(false),
        }
    }

    #[test]
    fn test_sample() {
        let input = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;
        let bounds = Rect {
            x: 0,
            y: 0,
            width: 11,
            height: 7,
        };
        match Day::parse(input, &bounds) {
            Some(day) => {
                let part1 = day.part1();
                assert!(part1 == 12, "Invalid solution");
            }
            None => assert!(false, "Invalid input"),
        }
    }
}
