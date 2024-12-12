use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::time::Instant;

type StoneMap = HashMap<i64, Stone>;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Stone {
    value: i64,
    count: i64,
}

fn split_value(value: i64) -> Option<(i64, i64)> {
    let mut tmp = value;
    let mut digits = 0;
    loop {
        tmp /= 10;
        digits += 1;
        if tmp == 0 {
            break;
        }
    }
    if digits & 1 == 1 {
        return None;
    }
    let half: i64 = 10_i64.pow(digits / 2);
    return Some((value / half, value % half));
}

impl Stone {
    fn simulate(mut self: Self, out_dict: &mut StoneMap) {
        if self.value == 0 {
            self.value = 1;
        } else if let Some(half) = split_value(self.value) {
            Stone {
                value: half.0,
                count: self.count,
            }
            .add_to(out_dict);
            self.value = half.1;
        } else {
            self.value *= 2024;
        }
        self.add_to(out_dict);
    }
    fn add_to(self: Self, dict: &mut StoneMap) {
        match dict.entry(self.value) {
            Entry::Occupied(o) => o.into_mut().count += self.count,
            Entry::Vacant(v) => {
                v.insert(self);
            }
        };
    }
    fn parse(s: &str) -> StoneMap {
        let mut ret: StoneMap = HashMap::new();
        for num in s.split(" ").filter_map(|n| n.parse::<i64>().ok()) {
            Stone {
                value: num,
                count: 1,
            }
            .add_to(&mut ret);
        }
        return ret;
    }
    fn count(map: &StoneMap) -> i64 {
        let mut ret: i64 = 0;
        for (_, s) in map.iter() {
            ret += s.count;
        }
        return ret;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    let input: String = fs::read_to_string("src/day11/input.txt")?;
    let mut stones = Stone::parse(&input);

    for _ in 0..25 {
        let mut solved: StoneMap = HashMap::new();
        for (_, stone) in stones {
            stone.simulate(&mut solved);
        }
        stones = solved;
    }
    println!("Part1: {}", Stone::count(&stones));

    for _ in 0..50 {
        let mut solved: StoneMap = HashMap::new();
        for (_, stone) in stones {
            stone.simulate(&mut solved);
        }
        stones = solved;
    }
    println!("Part2: {}", Stone::count(&stones));
    let elapsed = start_time.elapsed();
    println!("Run time: {:?}", elapsed);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse() {
        let stones = Stone::parse("125 17");
        let expected = HashMap::from([
            (
                125,
                Stone {
                    value: 125,
                    count: 1,
                },
            ),
            (
                17,
                Stone {
                    value: 17,
                    count: 1,
                },
            ),
        ]);
        assert!(stones == expected);
    }
    #[test]
    fn test_simulate() {
        let mut stones = Stone::parse("125 17");
        const EXPECTED: [&str; 6] = [
            "253000 1 7",
            "253 0 2024 14168",
            "512072 1 20 24 28676032",
            "512 72 2024 2 0 2 4 2867 6032",
            "1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32",
            "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2",
        ];
        for exp in EXPECTED {
            let mut solved: StoneMap = HashMap::new();
            for (_, stone) in stones {
                stone.simulate(&mut solved);
            }
            assert!(solved == Stone::parse(exp));
            stones = solved;
        }
    }
    #[test]
    fn test_split() {
        let result = split_value(123);
        assert!(result.is_none());
        let result = split_value(1234);
        assert!(result == Some((12, 34)));
    }
}
