use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;
use std::{fs, i64};

struct Day19 {
    towels: Vec<String>,
    designs: Vec<String>,
    solutions: Vec<i64>,
}

impl Day19 {
    fn parse(input: &str) -> Day19 {
        let mut ret = Day19 {
            towels: Vec::new(),
            designs: Vec::new(),
            solutions: Vec::new(),
        };
        let mut reading_towels = true;
        for line in input.lines() {
            if reading_towels {
                if line.is_empty() {
                    reading_towels = false;
                    continue;
                }
                ret.towels
                    .extend(line.split(",").map(|s| s.trim().to_string()));
            } else {
                if !line.is_empty() {
                    ret.designs.push(line.to_string());
                }
            }
        }
        return ret;
    }

    fn rec(&self, memo: &mut HashMap<String, i64>, design: &str) -> i64 {
        let mut ret: i64 = 0;
        for towel in self.towels.iter() {
            if design.starts_with(towel) {
                if design.len() == towel.len() {
                    ret += 1;
                    continue;
                }
                let slice = &design[towel.len()..];
                ret += match memo.entry(slice.to_string()) {
                    Entry::Occupied(o) => *o.get(),
                    Entry::Vacant(_) => self.rec(memo, slice),
                };
            }
        }
        memo.insert(design.to_string(), ret);
        return ret;
    }

    fn solve(&mut self) {
        if !self.solutions.is_empty() {
            return;
        }
        for design in self.designs.iter() {
            let mut memo: HashMap<String, i64> = HashMap::new();
            self.solutions.push(self.rec(&mut memo, &design[..]));
        }
    }

    fn part1(&self) -> i64 {
        if self.solutions.is_empty() {
            panic!("part1() was called before solve()");
        }
        self.solutions
            .iter()
            .fold(0_i64, |acc, v| if *v > 0 { acc + 1 } else { acc })
    }

    fn part2(&self) -> i64 {
        if self.solutions.is_empty() {
            panic!("part2() was called before solve()");
        }
        self.solutions.iter().fold(0_i64, |acc, v| acc + v)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    let mut solver = Day19::parse(&fs::read_to_string("src/day19/input.txt")?);
    solver.solve();
    println!("Part1: {}", solver.part1());
    println!("Part2: {}", solver.part2());

    let elapsed = start_time.elapsed();
    println!("Run time: {:?}", elapsed);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_example1() {
        let input = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#;

        let mut solver = Day19::parse(input);
        solver.solve();
        assert!(solver.part1() == 6);
        assert!(solver.part2() == 16);
    }
}
