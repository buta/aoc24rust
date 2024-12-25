use std::collections::{hash_map::Entry, HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::time::Instant;

use itertools::Itertools;

struct Day23 {
    lan: HashMap<String, HashSet<String>>,
}

impl Day23 {
    fn parse(input: &str) -> Day23 {
        let mut lan: HashMap<String, HashSet<String>> = HashMap::new();
        for line in input.lines() {
            if let [a, b] = line.split('-').collect::<Vec<&str>>()[..] {
                match lan.entry(a.to_string()) {
                    Entry::Occupied(mut o) => {
                        o.get_mut().insert(b.to_string());
                    }
                    Entry::Vacant(v) => {
                        v.insert(HashSet::from([b.to_string()]));
                    }
                }
                match lan.entry(b.to_string()) {
                    Entry::Occupied(mut o) => {
                        o.get_mut().insert(a.to_string());
                    }
                    Entry::Vacant(v) => {
                        v.insert(HashSet::from([a.to_string()]));
                    }
                }
            } else {
                panic!("Invalid input.");
            }
        }
        Day23 { lan }
    }

    fn part1(&self) -> i64 {
        let mut triangles: HashSet<(String, String, String)> = HashSet::new();
        for (k, v) in self.lan.iter() {
            if !k.starts_with("t") {
                continue;
            }
            for a in v.iter() {
                for b in v.iter() {
                    if a == b {
                        continue;
                    }
                    if self.lan[b].contains(a) {
                        let mut sorted = vec![k.to_string(), a.to_string(), b.to_string()];
                        sorted.sort();
                        let tuple = (sorted.remove(0), sorted.remove(0), sorted.remove(0));
                        triangles.insert(tuple);
                    }
                }
            }
        }
        triangles.len() as i64
    }

    fn part2(&self) -> String {
        let mut graphs;

        let mut ret = String::new();
        ret
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    let input = fs::read_to_string("src/day23/input.txt")?;

    let mut day = Day23::parse(&input);
    //let secrets: Vec<i64> =
    //    .lines()
    //    .filter_map(|n| n.parse::<i64>().ok())
    //    .collect();

    println!("Part1: {}", day.part1());

    println!("Part2: {}", -1);
    let elapsed = start_time.elapsed();
    println!("Run time: {:?}", elapsed);
    Ok(())
}

#[cfg(test)]
mod test {
    use std::fmt;

    use super::*;
    #[test]
    fn test_example1() {
        let input = "kh-tc\nqp-kh\nde-cg\nka-co\nyn-aq\nqp-ub\ncg-tb\nvc-aq\ntb-ka\nwh-tc\nyn-cg\nkh-ub\nta-co\nde-co\ntc-td\ntb-wq\nwh-td\nta-ka\ntd-qp\naq-cg\nwq-ub\nub-vc\nde-ta\nwq-aq\nwq-vc\nwh-yn\nka-de\nkh-ta\nco-tc\nwh-qp\ntb-vc\ntd-yn";
        let day = Day23::parse(input);
        assert!(day.part1() == 7);
    }

    #[test]
    fn test_example2() {}

    #[test]
    fn test_buffer() {}

    #[test]
    fn test_buffer_pack_unpack() {}
}
