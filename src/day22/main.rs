use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::time::Instant;
use std::{cmp::min, collections::hash_map::Entry};

const PRUNE: i64 = 16_777_216;

struct IntBuffer {
    value: i32,
    items: usize,
}

impl IntBuffer {
    fn new() -> IntBuffer {
        IntBuffer { value: 0, items: 0 }
    }
    fn push(&mut self, item: i8) {
        self.value <<= 8;
        self.value |= item as i32 & 0xFF;
        self.items += 1;
    }

    fn get(&self) -> i32 {
        self.value
    }

    fn len(&self) -> usize {
        min(self.items, 4)
    }

    fn _unpack(packed: i32) -> String {
        let mut ret = String::new();
        ret.push_str("(");
        let mut tmp: i64 = packed as i64;
        let mut first = true;
        let bytes = packed.to_le_bytes();
        for i in (0..4).rev() {
            let digit = i8::from_le_bytes([bytes[i]]);
            if first {
                first = false;
            } else {
                ret.push_str(", ");
            }
            ret.push_str(&digit.to_string());
            tmp = (tmp & 0xFFFFFF) << 8;
        }
        ret.push_str(")");
        return ret;
    }

    fn _pack(digits: &[i8]) -> i32 {
        let mut ret: i32 = 0;
        for d in digits {
            ret <<= 8;
            ret |= *d as i32 & 0xFF;
        }
        return ret;
    }
}

struct CalcResult {
    number: i64,
    patterns: HashMap<i32, i32>,
}

fn calculate(secret: i64, rounds: i64) -> CalcResult {
    let mut patterns: HashMap<i32, i32> = HashMap::new();
    let mut seq_buf = IntBuffer::new();
    let mut num = secret;
    let mut last_price = num.rem_euclid(10) as i8;
    for _ in 0..rounds {
        num ^= num * 64;
        num = num.rem_euclid(PRUNE);
        num ^= num / 32;
        num = num.rem_euclid(PRUNE);
        num ^= num * 2048;
        num = num.rem_euclid(PRUNE);
        let price = num.rem_euclid(10) as i8;
        seq_buf.push(price - last_price);

        last_price = price;
        if seq_buf.len() == 4 {
            match patterns.entry(seq_buf.get()) {
                Entry::Occupied(_) => { /* ignore */ }
                Entry::Vacant(v) => {
                    v.insert(price as i32);
                }
            }
        }
    }
    CalcResult {
        number: num,
        patterns,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();

    let secrets: Vec<i64> = fs::read_to_string("src/day22/input.txt")?
        .lines()
        .filter_map(|n| n.parse::<i64>().ok())
        .collect();

    let results: Vec<CalcResult> = secrets.iter().map(|s| calculate(*s, 2000)).collect();

    println!(
        "Part1: {}",
        results.iter().fold(0_i64, |acc, r| acc + r.number)
    );

    let mut all_patterns: HashSet<i32> = HashSet::new();
    for r in results.iter() {
        all_patterns.extend(r.patterns.keys());
    }

    let mut best_sum = 0;
    for pattern in all_patterns {
        let pat_sum = results.iter().fold(0_i64, |acc, r| {
            acc + (*r.patterns.get(&pattern).unwrap_or(&0) as i64)
        });
        if pat_sum > best_sum {
            best_sum = pat_sum;
        }
    }
    println!("Part2: {}", best_sum);
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
        let expected = vec![
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];
        for (i, exp) in expected.into_iter().enumerate() {
            let result = calculate(123, i as i64 + 1);
            assert!(result.number == exp);
        }
    }

    #[test]
    fn test_example2() {
        let input = vec![1, 2, 3, 2024];
        let results: Vec<CalcResult> = input.iter().map(|i| calculate(*i, 2000)).collect();
        let exp: i32 = IntBuffer::_pack(&[-2, 1, -1, 3]);
        let mut sum = 0;
        for r in results.iter() {
            let res = r.patterns.get(&exp).unwrap_or(&0);
            sum += res;
        }
        let res = results
            .iter()
            .fold(0, |acc, r| acc + r.patterns.get(&exp).unwrap_or(&0));
        assert!(res == 23);
    }

    #[test]
    fn test_buffer() {
        let mut rb = IntBuffer::new();
        let test_values = vec![1, 2, 3, 4, 5, 6, 7];
        let items = rb.get();
        assert!(items == 0);
        assert!(rb.len() == 0);
        let concat_numbers =
            |items: &[i8]| -> i32 { items.iter().fold(0, |acc, i| acc * 256 + *i as i32) };

        rb.push(test_values[0]);
        let items = rb.get();
        let expected = concat_numbers(&test_values[0..1]);
        assert!(items == expected);
        assert!(rb.len() == 1);

        rb.push(test_values[1]);
        let items = rb.get();
        let expected = concat_numbers(&test_values[0..2]);
        assert!(items == expected);
        assert!(rb.len() == 2);

        rb.push(test_values[2]);
        let items = rb.get();
        assert!(items == concat_numbers(&test_values[0..3]));
        assert!(rb.len() == 3);

        rb.push(test_values[3]);
        let items = rb.get();
        assert!(items == concat_numbers(&test_values[0..4]));
        assert!(rb.len() == 4);

        rb.push(test_values[4]);
        let items = rb.get();
        assert!(items == concat_numbers(&test_values[1..5]));
        assert!(rb.len() == 4);
    }

    #[test]
    fn test_buffer_pack_unpack() {
        const MIN: i8 = -9;
        const MAX: i8 = 10;
        for i in MIN..MAX {
            for j in MIN..MAX {
                for k in MIN..MAX {
                    for l in MIN..MAX {
                        let packed = IntBuffer::_pack(&[i, j, k, l]);
                        let unpacked = IntBuffer::_unpack(packed);
                        let formatted = format!("({i}, {j}, {k}, {l})");
                        assert!(unpacked == formatted);
                    }
                }
            }
        }
    }
}
