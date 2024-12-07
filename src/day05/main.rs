use std::cmp;
use std::collections::HashSet;
use std::collections::{hash_map::Entry, HashMap};
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    let mut rules: HashMap<i64, HashSet<i64>> = HashMap::new();
    let mut pages: Vec<Vec<i64>> = Vec::new();
    {
        let file = File::open("src/day05/input.txt")?;
        let reader = BufReader::new(file);
        let mut read_rules = true;
        for line_rs in reader.lines() {
            let line = line_rs?;
            if read_rules {
                if line.len() == 0 {
                    read_rules = false;
                    continue;
                }
                let res: Vec<i64> = line.split("|").map(|n| n.parse::<i64>().unwrap()).collect();
                let set = match rules.entry(res[0]) {
                    Entry::Occupied(o) => o.into_mut(),
                    Entry::Vacant(v) => v.insert(HashSet::new()),
                };
                set.insert(res[1]);
            } else {
                let tmp: Vec<i64> = line.split(",").map(|n| n.parse::<i64>().unwrap()).collect();
                pages.push(tmp);
            }
        }
    }

    let check_part1 = |page: &Vec<i64>| -> Option<i64> {
        (page.len() > 0).then(|| ())?;
        let mut before: Vec<i64> = Vec::new();
        for num in page {
            match rules.get(&num) {
                Some(set) => {
                    for r in set {
                        if before.contains(r) {
                            return None;
                        }
                    }
                }
                None => {}
            }
            before.push(*num);
        }
        return page.get(page.len() / 2).cloned();
    };

    let fix_part2 = |page: &Vec<i64>| -> Vec<i64> {
        let mut ret: Vec<i64> = Vec::new();
        for num in page {
            let mut idx = ret.len();
            match rules.get(num) {
                Some(rule_set) => {
                    for r in rule_set {
                        idx = cmp::min(ret.iter().position(|e| e == r).unwrap_or(idx), idx);
                    }
                }
                None => {}
            }
            ret.insert(idx, *num);
        }
        return ret;
    };

    let mut part1: i64 = 0;
    let mut part2: i64 = 0;

    for page in pages {
        match check_part1(&page) {
            Some(num) => part1 += num,
            None => {
                let fixed = fix_part2(&page);
                match check_part1(&fixed) {
                    Some(num) => part2 += num,
                    None => panic!("Couldn't fix page order?"),
                }
            }
        }
    }

    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
    let elapsed = start_time.elapsed();
    println!("Run time: {:?}", elapsed);

    Ok(())
}
