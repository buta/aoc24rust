use lazy_static::lazy_static;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::time::Instant;

type Operation = Box<fn(i32, i32) -> i32>;

lazy_static! {
    static ref OPERATIONS: HashMap<String, Operation> = {
        let mut ret: HashMap<String, Operation> = HashMap::new();
        ret.insert(String::from("AND"), Box::new(|a: i32, b: i32| a & b));
        ret.insert(String::from("OR"), Box::new(|a: i32, b: i32| a | b));
        ret.insert(String::from("XOR"), Box::new(|a: i32, b: i32| a ^ b));
        ret
    };
}
struct Gate {
    a: String,
    b: String,
    out: String,
    op: Operation,
    done: bool,
}
struct Day24 {
    gates: Vec<Gate>,
    levels: HashMap<String, i32>,
    outputs: HashMap<String, Option<i32>>,
}

impl Day24 {
    fn parse(input: &str) -> Day24 {
        let mut ret = Day24 {
            gates: Vec::new(),
            levels: HashMap::new(),
            outputs: HashMap::new(),
        };
        let mut parsing_levels = true;
        for line in input.lines() {
            if parsing_levels {
                if line.is_empty() {
                    parsing_levels = false;
                    continue;
                }
                let mut it = line.split(":").into_iter();
                let key = it.next().expect("Missing input name").to_string();
                let value = it
                    .next()
                    .expect("Missing input level")
                    .trim()
                    .parse::<i32>()
                    .expect("Input level is not an integer.");
                ret.levels.insert(key, value);
            } else {
                let mut it = line.split(" ").into_iter();
                let a = it.next().expect("Gate A is missing").to_string();
                let op = OPERATIONS[it.next().expect("Gate type is missing")].clone();
                let b = it.next().expect("Gate B is missing").to_string();
                assert!(it.next() == Some("->"), "Invalid gate input");
                let out = it.next().expect("Gate out is missing").to_string();
                if out.starts_with("z") {
                    ret.outputs.insert(out.clone(), None);
                }
                ret.gates.push(Gate {
                    a,
                    b,
                    out,
                    op,
                    done: false,
                });
            }
        }
        ret
    }

    fn part1(&mut self) -> i64 {
        while self.outputs.iter().any(|(_, v)| v.is_none()) {
            for gate in self.gates.iter_mut() {
                if !gate.done {
                    let a = self.levels.get(&gate.a);
                    let b = self.levels.get(&gate.b);
                    if let (Some(a), Some(b)) = (a, b) {
                        let result = gate.op.as_ref()(*a, *b);
                        gate.done = true;
                        self.levels.insert(gate.out.clone(), result);
                        if gate.out.starts_with("z") {
                            self.outputs.insert(gate.out.clone(), Some(result));
                        }
                    }
                }
            }
        }
        let mut ret: i64 = 0;
        for i in 0.. {
            let key = format!("z{:02}", i);
            match self.outputs.get(&key) {
                Some(Some(level)) => {
                    let tmp = *level as i64;
                    ret |= tmp << i;
                }
                _ => {
                    break;
                }
            }
        }
        ret
    }

    fn part2(&self) -> String {
        String::new()
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    let input = fs::read_to_string("src/day24/input.txt")?;
    let mut day = Day24::parse(&input);
    println!("Part1: {}", day.part1());
    println!("Part2: {}", day.part2());
    let elapsed = start_time.elapsed();
    println!("Run time: {:?}", elapsed);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_example1() {
        let input = r#"x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02"#;
        let mut day = Day24::parse(input);
        assert!(day.part1() == 4);
        //assert!(day.part2() == "co,de,ka,ta");
    }
}
