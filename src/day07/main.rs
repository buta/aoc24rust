use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::time::Instant;

type Operation = fn(i64, i64) -> i64;

struct Solver {
    solution: i64,
    numbers: Vec<i64>,
}

impl Solver {
    fn new(line: &str) -> Option<Solver> {
        let mut parts = line.split(": ").fuse();
        let sol = parts.next()?.parse::<i64>().ok()?;
        let nums: Vec<_> = parts
            .next()?
            .split(" ")
            .map(|v| v.parse::<i64>().ok())
            .collect();
        if nums.len() < 2 || nums.iter().any(|v| v.is_none()) {
            return None;
        }
        return Some(Solver {
            solution: sol,
            numbers: nums.iter().map(|v| v.unwrap()).collect(),
        });
    }

    fn rec<'a>(&self, ops: &'a Vec<Operation>, i: usize, sum: i64) -> bool {
        if sum > self.solution {
            return false;
        }
        if i == self.numbers.len() - 1 {
            return sum == self.solution;
        }
        match self.numbers.get(i + 1) {
            Some(b) => return ops.iter().any(|v| self.rec(&ops, i + 1, v(sum, *b))),
            None => return false,
        }
    }

    fn solve<'a>(&self, ops: &'a Vec<Operation>) -> bool {
        return match self.numbers.get(0) {
            Some(n) => self.rec(&ops, 0, *n),
            None => false,
        };
    }
}

fn mult(a: i64, b: i64) -> i64 {
    return a * b;
}

fn add(a: i64, b: i64) -> i64 {
    return a + b;
}

fn concat(a: i64, b: i64) -> i64 {
    let mut tmp = b;
    let mut tmp2 = a;
    loop {
        tmp /= 10;
        tmp2 *= 10;
        if tmp == 0 {
            break;
        }
    }
    return tmp2 + b;
}

impl Display for Solver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.solution, self.numbers)
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();

    let file = File::open("src/day07/input.txt")?;
    let reader = BufReader::new(file);
    let mut equations: Vec<Solver> = Vec::new();
    let part1_ops: Vec<Operation> = [mult, add].to_vec();
    let part2_ops: Vec<Operation> = [mult, add, concat].to_vec();

    for line_rs in reader.lines() {
        let line = line_rs.expect("Line is missing");
        let s = Solver::new(&line);
        match s {
            Some(s) => equations.push(s),
            None => {}
        }
    }
    let mut part1: i64 = 0;
    let mut part2: i64 = 0;
    for s in equations {
        if s.solve(&part1_ops) {
            part1 += s.solution;
            part2 += s.solution;
        } else if s.solve(&part2_ops) {
            part2 += s.solution;
        }
    }
    let elapsed = start_time.elapsed();
    println!("Part1: {}\nPart2: {}", part1, part2);
    println!("Run time: {:?}", elapsed);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_concat() {
        assert_eq!(concat(1, 2), 12);
        assert_eq!(concat(1, 0), 10);
        assert_eq!(concat(0, 1), 1);
        assert_eq!(concat(10, 10), 1010);
        assert_eq!(concat(1234, 1234), 12341234);
        assert_eq!(concat(1230, 1230), 12301230);
        assert_eq!(concat(2843285760, 74), 284328576074);
    }
}
