use std::{borrow::BorrowMut, cell::RefCell, error::Error, fs, rc::Rc, time::Instant};

use aoc24rust::utils::{PointT, RectT};

type Point = PointT<i64>;
type Rect = RectT<i64>;

const CLEAR: char = '.';
const WALL: char = '#';
const CRATE: char = 'O';
const ROBOT: char = '@';

trait Block {
    fn try_push(&self, dir: Point) -> bool;
    fn push(&mut self, dir: Point);
}
struct Wall {}
impl Block for Wall {
    fn try_push(&self, _: Point) -> bool {
        false
    }

    fn push(&mut self, _: Point) {
        panic!("Do not ever call this");
    }
}
struct Crate {
    day: Rc<RefCell<Day15>>,
    position: Point,
    width: usize,
}
impl Block for Crate {
    fn try_push(&self, dir: Point) -> bool {
        true
    }
    fn push(&mut self, dir: Point) {
        todo!()
    }
}

struct Day15 {
    bounds: Rect,
    blocks: Vec<Rc<RefCell<dyn Block>>>,
    level: Vec<Vec<Option<Rc<RefCell<dyn Block>>>>>,
}

impl Day15 {
    fn parse(input: &str, width: usize) -> Rc<RefCell<Day15>> {
        let mut ret = Rc::new(RefCell::new(Day15 {
            bounds: Rect {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            },
            blocks: Vec::new(),
            level: Vec::new(),
        }));
        //let mut blocks: Vec<Rc<dyn Block>> = Vec::new();
        //let mut level: Vec<Vec<Option<Rc<dyn Block>>>> = Vec::new();
        let mut parsing_level = true;
        //for (y, line) in input.lines().enumerate() {
        //    if parsing_level {
        //        if line.is_empty() {
        //            parsing_level = false;
        //
        //            //ret.bounds.height = y as i64;
        //        }
        //        for (x, c) in line.chars().enumerate() {
        //            match c {
        //                CLEAR => {}
        //                WALL => {
        //                    let wall = Rc::new(RefCell::new(Wall {}));
        //                    let mut tmp = ret.get_mut();
        //                    tmp.blocks.push(wall.clone());
        //                    //ret.borrow_mut().borrow().level[y][x] = Some(wall);
        //                }
        //                CRATE => {
        //                    let crate_ = Rc::new(RefCell::new(Crate {
        //                        day: ret,
        //                        position: Point {
        //                            x: x as i64,
        //                            y: y as i64,
        //                        },
        //                        width,
        //                    }));
        //                    //ret.get_mut().blocks.push(crate_.clone());
        //                    //ret.get_mut().level[y][x] = Some(crate_);
        //                }
        //                ROBOT => {}
        //                c => panic!("Invalid character in map."),
        //            }
        //        }
        //    } else {
        //    }
        //}
        return ret;
    }

    fn simulate(&mut self) {}
}

fn main() -> Result<(), Box<dyn Error>> {
    let start_time: Instant = Instant::now();
    let input = &fs::read_to_string("src/day14/input.txt")?.replace("\r\n", "\n");
    let mut part1 = Day15::parse(input, 1);
    //part1.borrow_mut().simulate();

    let mut part2 = Day15::parse(input, 2);
    //{
    //    Some(day) => {
    //        //println!("Part1: {}", day.part1());
    //        //println!("Part2: {}", day.part2());
    //    }
    //    None => panic!("Invalid input."),
    //}
    let elapsed = start_time.elapsed();
    println!("Run time: {:?}", elapsed);
    todo!("finish day15");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse() {}
}
