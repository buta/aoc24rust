use std::error::Error;
use std::fs;
use std::time::Instant;

const BLOCK: char = '#';
const SPACE: char = '.';
const MAX_HEIGHT: i32 = 5;

struct Day25 {
    locks: Vec<Vec<i32>>,
    keys: Vec<Vec<i32>>,
}

impl Day25 {
    fn parse(input: &str) -> Day25 {
        let mut ret = Day25 {
            locks: Vec::new(),
            keys: Vec::new(),
        };
        for part in input.split("\n\n") {
            let shape: Vec<Vec<char>> = part.lines().map(|l| l.chars().collect()).collect();
            let height = shape.len();
            let width = shape[0].len();
            let mut pattern: Vec<i32> = Vec::new();
            if shape[0].iter().all(|c| *c == BLOCK) {
                // lock
                for x in 0..width {
                    for y in 1..height {
                        if shape[y][x] == SPACE {
                            pattern.push(y as i32 - 1);
                            break;
                        }
                    }
                }
                ret.locks.push(pattern);
            } else {
                // key
                for x in 0..width {
                    for y in (0..height).rev() {
                        if shape[y][x] == SPACE {
                            pattern.push(height as i32 - y as i32 - 2);
                            break;
                        }
                    }
                }
                ret.keys.push(pattern);
            }
        }
        ret
    }

    fn part1(&self) -> i64 {
        let mut ret = 0;
        for lock in self.locks.iter() {
            for key in self.keys.iter() {
                if lock
                    .iter()
                    .zip(key.iter())
                    .all(|(l, k)| l + k <= MAX_HEIGHT)
                {
                    ret += 1;
                }
            }
        }
        ret
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    let input = fs::read_to_string("src/day25/input.txt")?;
    let day = Day25::parse(&input);
    println!("Part1: {}", day.part1());
    let elapsed = start_time.elapsed();
    println!("Run time: {:?}", elapsed);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_example1() {
        let input = fs::read_to_string("src/day25/dbg.txt").unwrap();
        let day = Day25::parse(&input);
        assert!(
            day.keys
                == vec![
                    vec![5, 0, 2, 1, 3],
                    vec![4, 3, 4, 0, 2],
                    vec![3, 0, 2, 0, 1]
                ]
        );
        assert!(day.locks == vec![vec![0, 5, 3, 4, 3], vec![1, 2, 0, 5, 3]]);
        assert!(day.part1() == 3);
    }
}
