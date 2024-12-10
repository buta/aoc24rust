use std::fs;
use std::time::Instant;
use std::{collections::HashSet, error::Error};

use aoc24rust::utils::{Point, Rect};

const TRAIL_HEAD: u8 = 0;
const TRAIL_END: u8 = 9;

const DIRECTIONS: [Point<i32>; 4] = [
    Point { x: 0, y: 1 },
    Point { x: 1, y: 0 },
    Point { x: 0, y: -1 },
    Point { x: -1, y: 0 },
];

fn walk(p: &Point<i32>, map: &Vec<Vec<u8>>, bounds: &Rect<i32>) -> Vec<Point<i32>> {
    let mut ret = Vec::new();
    let height = map[p.y as usize][p.x as usize];
    if height == TRAIL_END {
        //println!("Trail end found at: {}", &p);
        ret.push(p.clone());
        return ret;
    }
    for dir in DIRECTIONS.iter() {
        let target = p.clone() + dir.clone();
        if bounds.is_inside(&target) {
            let t_height = map[target.y as usize][target.x as usize];
            if t_height == height + 1 {
                ret.extend(walk(&target, &map, &bounds));
            }
        }
    }
    return ret;
}

fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    let mut map: Vec<Vec<u8>> = Vec::new();
    let input: String = fs::read_to_string("src/day10/input.txt")?;
    for line in input.lines() {
        let mut tmp: Vec<u8> = Vec::new();
        for c in line.chars() {
            match c.to_digit(10) {
                Some(number) => tmp.push(number as u8),
                None => tmp.push(u8::MAX),
            }
        }
        map.push(tmp);
    }
    assert!(
        map.iter().all(|v| v.len() == map[0].len()),
        "Map width is not uniform"
    );
    let bounds = Rect {
        x: 0 as i32,
        y: 0 as i32,
        width: map[0].len() as i32,
        height: map.len() as i32,
    };

    let mut part1: i64 = 0;
    let mut part2: i64 = 0;
    for x in 0..map[0].len() {
        for y in 0..map.len() {
            if map[y][x] == TRAIL_HEAD {
                let p = Point {
                    x: x as i32,
                    y: y as i32,
                };
                let found = walk(&p, &map, &bounds);
                let ends = found.iter().collect::<HashSet<_>>().iter().count() as i64;
                let paths = found.len() as i64;
                //println!("Results for {}: ends={} paths={}", p, ends, paths);
                part1 += ends;
                part2 += paths;
            }
        }
    }

    let elapsed = start_time.elapsed();
    println!("Part1: {}\nPart2: {}", part1, part2);
    println!("Run time: {:?}", elapsed);
    Ok(())
}
