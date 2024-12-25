use aoc24rust::utils::PointT;
use itertools::Itertools;
use lazy_static::lazy_static;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
    time::Instant,
};

type Point = PointT<i32>;
type Memo = HashMap<(i32, Vec<char>), i64>;

const UP: char = '^';
const DOWN: char = 'v';
const LEFT: char = '<';
const RIGHT: char = '>';
const ACTION: char = 'A';

const NUMPAD_START: Point = Point { x: 2, y: 3 };
const DIRPAD_START: Point = Point { x: 2, y: 0 };

lazy_static! {
    static ref NUMPAD: HashMap<char, Point> = {
        let mut ret = HashMap::new();
        ret.insert('7', Point { x: 0, y: 0 });
        ret.insert('8', Point { x: 1, y: 0 });
        ret.insert('9', Point { x: 2, y: 0 });
        ret.insert('4', Point { x: 0, y: 1 });
        ret.insert('5', Point { x: 1, y: 1 });
        ret.insert('6', Point { x: 2, y: 1 });
        ret.insert('1', Point { x: 0, y: 2 });
        ret.insert('2', Point { x: 1, y: 2 });
        ret.insert('3', Point { x: 2, y: 2 });
        ret.insert('0', Point { x: 1, y: 3 });
        ret.insert(ACTION, NUMPAD_START);
        ret
    };
    static ref DIRPAD: HashMap<char, Point> = {
        let mut ret = HashMap::new();
        ret.insert(UP, Point { x: 1, y: 0 });
        ret.insert(ACTION, DIRPAD_START);
        ret.insert(LEFT, Point { x: 0, y: 1 });
        ret.insert(DOWN, Point { x: 1, y: 1 });
        ret.insert(RIGHT, Point { x: 2, y: 1 });
        ret
    };
    static ref NUMPAD_VALID: HashSet<Point> = HashSet::from_iter(NUMPAD.values().cloned());
    static ref DIRPAD_VALID: HashSet<Point> = HashSet::from_iter(DIRPAD.values().cloned());
    static ref DIR_TO_POINT: HashMap<char,Point> = {
        let mut ret = HashMap::new();
        ret.insert(UP, Point{x:0,y:-1});
        ret.insert(RIGHT, Point{x:1,y:0});
        ret.insert(DOWN, Point{x:0,y:1});
        ret.insert(LEFT, Point{x:-1,y:0});
        ret
    };
    //static ref MEMO: HashMap<(i32, String), i64> = HashMap::new();
}

fn is_path_valid(start: &Point, path: &Vec<char>, valid: &HashSet<Point>) -> bool {
    let mut p = start.clone();
    for c in path.iter() {
        match DIR_TO_POINT.get(c) {
            Some(d) => {
                p = p + d.clone();
                if !valid.contains(&p) {
                    return false;
                }
            }
            None => panic!("Invalid char in path"),
        }
    }
    true
}

fn delta_to_path(delta: Point) -> Vec<char> {
    let mut ret = Vec::new();
    for _ in 0..delta.x.abs() {
        ret.push(if delta.x > 0 { RIGHT } else { LEFT })
    }
    for _ in 0..delta.y.abs() {
        ret.push(if delta.y > 0 { DOWN } else { UP })
    }
    return ret;
}

fn rec_dir(memo: &mut Memo, depth: i32, path: Vec<char>) -> i64 {
    if let Some(ret) = memo.get(&(depth, path.clone())) {
        return *ret;
    }
    let mut ret: i64 = 0;
    if depth <= 0 {
        ret = path.len() as i64;
    } else {
        let mut cursor = DIRPAD_START;
        let mut last_char: Option<char> = None;
        for c in path.iter() {
            if last_char == Some(*c) {
                // press A again
                ret += rec_dir(memo, depth - 1, vec![ACTION]);
                continue;
            }
            let target = DIRPAD.get(c).expect("Invalid dirpad character.");
            let path = delta_to_path(target.clone() - cursor.clone());
            let len = path.len();
            ret += path
                .into_iter()
                .permutations(len)
                .filter_map(|mut i| {
                    if is_path_valid(&cursor, &i, &DIRPAD_VALID) {
                        i.push(ACTION);
                        Some(rec_dir(memo, depth - 1, i))
                    } else {
                        None
                    }
                })
                .min()
                .expect("Invalid input? Not a single valid dir path.");
            cursor = target.clone();
            last_char = Some(*c);
        }
    }
    memo.insert((depth, path.clone()), ret);
    ret
}

fn solve_line(code: &str, depth: i32) -> i64 {
    let mut ret: i64 = 0;
    let mut cursor = NUMPAD_START;
    let mut memo: Memo = HashMap::new();
    for c in code.chars() {
        let target = NUMPAD.get(&c).expect("Invalid numpad character.");
        let path = delta_to_path(target.clone() - cursor.clone());
        let len = path.len();
        ret += path
            .into_iter()
            .permutations(len)
            .filter_map(|mut i| {
                if is_path_valid(&cursor, &i, &NUMPAD_VALID) {
                    i.push(ACTION);
                    Some(rec_dir(&mut memo, depth, i))
                } else {
                    None
                }
            })
            .min()
            .expect("Invalid input? Not a single valid path.");
        cursor = target.clone();
    }
    ret
}

fn solve(input: &str, depth: i32) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    let nums: Vec<i64> = lines
        .iter()
        .filter_map(|n| {
            n.chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .parse::<i64>()
                .ok()
        })
        .collect();
    let lengths: Vec<i64> = lines.iter().map(|n| solve_line(n, depth)).collect();
    //println!("Lengths:{:?}", lengths);
    nums.iter()
        .zip(lengths.iter())
        .fold(0_i64, |acc, (num, len)| acc + *num * *len)
}

fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    let input = fs::read_to_string("src/day21/input.txt")?;

    println!("Part1: {}", solve(&input, 2));
    println!("Part2: {}", solve(&input, 25));
    let elapsed = start_time.elapsed();
    println!("Run time: {:?}", elapsed);
    Ok(())
}
#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_example1() {
        let input = r#"029A
980A
179A
456A
379A"#;
        let solution = solve(&input, 2);
        assert!(solution == 126384);
        let sol2 = solve(&input, 25);
        assert!(sol2 == 154115708116294);
    }
}
