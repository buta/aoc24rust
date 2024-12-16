use std::{error::Error, fs, time::Instant};

use aoc24rust::utils::{PointT, RectT};

type Point = PointT<i64>;
type Rect = RectT<i64>;

enum Tile {
    Box(),
}

struct Day15 {
    //bounds: Rect,
    //level: Vec<Vec<Option>>,
    //boxes: Vec,
}

fn main() -> Result<(), Box<dyn Error>> {
    /*let start_time = Instant::now();
    let bounds = Rect {
        x: 0,
        y: 0,
        width: 101,
        height: 103,
    };
    match Day15::parse(&fs::read_to_string("src/day14/input.txt")?, &bounds) {
        Some(day) => {
            println!("Part1: {}", day.part1());
            println!("Part2: {}", day.part2());
        }
        None => panic!("Invalid input."),
    }
    let elapsed = start_time.elapsed();
    println!("Run time: {:?}", elapsed);*/
    todo!("finish day15");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse() {}
}
