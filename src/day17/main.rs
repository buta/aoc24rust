use std::error::Error;
use std::fs;
use std::time::Instant;

struct Machine {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    i_ptr: usize,
    program: Vec<i32>,
    out: Vec<i32>,
}

impl Machine {
    fn parse(input: &str) -> Option<Machine> {
        let mut a: Option<i64> = None;
        let mut b: Option<i64> = None;
        let mut c: Option<i64> = None;
        let mut p: Vec<i32> = Vec::new();
        for line in input.lines() {
            match line.split(": ").collect::<Vec<&str>>()[..] {
                ["Register A", val_str] => a = val_str.parse().ok(),
                ["Register B", val_str] => b = val_str.parse().ok(),
                ["Register C", val_str] => c = val_str.parse().ok(),
                ["Program", val_str] => p.extend(
                    val_str
                        .split(",")
                        .map(|v| v.parse::<i32>())
                        .flatten()
                        .collect::<Vec<i32>>(),
                ),
                _ => {}
            }
        }
        if let (Some(a), Some(b), Some(c)) = (a, b, c) {
            return Some(Machine {
                reg_a: a,
                reg_b: b,
                reg_c: c,
                i_ptr: 0,
                program: p,
                out: Vec::new(),
            });
        }
        None
    }

    fn combo(&self, value: i32) -> i64 {
        return match value {
            0..=3 => value as i64,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("Invalid combo"),
        };
    }

    fn op_adv(&mut self, value: i32) {
        self.i_ptr += 2;
        self.reg_a = self.reg_a / 2_i64.pow(self.combo(value) as u32);
    }

    fn op_bxl(&mut self, value: i32) {
        self.i_ptr += 2;
        let result = self.reg_b ^ value as i64;
        self.reg_b = result;
    }

    fn op_bst(&mut self, value: i32) {
        self.i_ptr += 2;
        self.reg_b = self.combo(value).rem_euclid(8);
    }
    fn op_jnz(&mut self, value: i32) {
        if self.reg_a == 0 {
            self.i_ptr += 2;
        } else {
            self.i_ptr = value as usize
        }
    }

    fn op_bxc(&mut self, _: i32) {
        self.i_ptr += 2;
        let result = self.reg_b ^ self.reg_c;
        self.reg_b = result;
    }

    fn op_out(&mut self, value: i32) {
        self.i_ptr += 2;
        let result = self.combo(value).rem_euclid(8);
        self.out.push(result as i32);
    }

    fn op_bdv(&mut self, value: i32) {
        self.i_ptr += 2;
        self.reg_b = self.reg_a / 2_i64.pow(self.combo(value) as u32);
    }
    fn op_cdv(&mut self, value: i32) {
        self.i_ptr += 2;
        self.reg_c = self.reg_a / 2_i64.pow(self.combo(value) as u32);
    }
    fn run(&mut self) {
        while self.i_ptr < self.program.len() {
            let value = self.program[self.i_ptr + 1];
            match self.program[self.i_ptr] {
                0 => self.op_adv(value),
                1 => self.op_bxl(value),
                2 => self.op_bst(value),
                3 => self.op_jnz(value),
                4 => self.op_bxc(value),
                5 => self.op_out(value),
                6 => self.op_bdv(value),
                7 => self.op_cdv(value),
                _ => panic!("Invalid opcode"),
            }
        }
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    match Machine::parse(&fs::read_to_string("src/day17/input.txt")?) {
        Some(mut m) => {
            m.run();
            println!("Part1: {:?}", &m.out);
            let mut i: i64 = 0;
            loop {
                m.out.clear();
                m.reg_a = i;
                m.reg_b = 0;
                m.reg_c = 0;
                m.i_ptr = 0;
                m.run();
                if m.out == m.program {
                    println!("Part2: {}", i);
                    break;
                }
                if m.program.ends_with(&m.out) {
                    i <<= 3;
                    continue;
                }
                i += 1;
            }
        }
        None => panic!("Invalid input."),
    }
    let elapsed = start_time.elapsed();
    println!("Run time: {:?}", elapsed);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_example1() {
        let input = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;
        match Machine::parse(input) {
            Some(mut m) => {
                m.run();
                assert!(m.out == vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
            }
            None => assert!(false, "Invalid input"),
        }
    }

    #[test]
    fn test_example2() {
        let input = r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"#;
        match Machine::parse(input) {
            Some(mut m) => {
                let mut solution: i64 = 0;
                for i in 0_i64.. {
                    m.out.clear();
                    m.reg_a = i;
                    m.reg_b = 0;
                    m.reg_c = 0;
                    m.i_ptr = 0;
                    m.run();
                    if m.out == m.program {
                        solution = i;
                        break;
                    }
                }
                assert!(solution == 117440);
            }
            None => assert!(false, "Invalid input"),
        }
    }
}
