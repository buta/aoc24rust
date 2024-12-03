use std::fs;
use std::io;
use std::time::Instant;

const MULT: &str = "mul(";
const DO: &str = "do()";
const DONT: &str = "don't()";
enum Command {
    MULT(i64, i64),
    DO,
    DONT,
}

struct Tokenizer {
    data: String,
    index: usize,
}

impl Tokenizer {
    fn new(data: String) -> Tokenizer {
        Tokenizer {
            data: data,
            index: 0,
        }
    }
}

impl Iterator for Tokenizer {
    type Item = Command;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.data.len() {
            if self.data[self.index..].starts_with(MULT) {
                self.index += MULT.len();
                let mut extract_char = |end: char| {
                    for i in 0..4 {
                        let chr = self.data.chars().nth(self.index + i).unwrap();
                        if chr.is_digit(10) {
                        } else if chr == end {
                            if i > 0 {
                                let ret = self.data[self.index..self.index + i].parse::<i64>().ok();
                                self.index += i + 1;
                                return ret;
                            }
                            break;
                        } else {
                            break;
                        }
                    }
                    return None;
                };
                let a: Option<i64> = extract_char(',');
                let b: Option<i64> = extract_char(')');
                if a.is_some() && b.is_some() {
                    return Some(Command::MULT(a?, b?));
                }
            } else if self.data[self.index..].starts_with(DO) {
                self.index += DO.len();
                return Some(Command::DO);
            } else if self.data[self.index..].starts_with(DONT) {
                self.index += DONT.len();
                return Some(Command::DONT);
            }
            self.index += 1;
        }
        return None;
    }
}

fn main() -> io::Result<()> {
    let start_time = Instant::now();
    let message: String = fs::read_to_string("src/day03/input01.txt")?;
    let tokenizer = Tokenizer::new(message);

    let mut sum1: i64 = 0;
    let mut sum2: i64 = 0;
    let mut is_active = true;
    for t in tokenizer {
        match t {
            Command::MULT(a, b) => {
                sum1 += a * b;
                if is_active {
                    sum2 += a * b;
                }
            }
            Command::DO => {
                is_active = true;
            }
            Command::DONT => {
                is_active = false;
            }
        }
    }
    println!("Part1: {}", sum1);
    println!("Part2: {}", sum2);
    let elapsed = start_time.elapsed();
    println!("Run time: {:?}", elapsed);
    Ok(())
}
