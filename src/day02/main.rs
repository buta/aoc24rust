use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::time::Instant;

fn check(numbers: &Vec<i64>) -> Option<usize> {
    let mut prev_num: Option<i64> = None;
    let mut prev_diff: Option<i64> = None;
    for (i, num) in numbers.iter().enumerate() {
        if prev_num.is_some() {
            let diff = prev_num.unwrap() - num;
            if diff == 0 || diff.abs() > 3 {
                return Some(i);
            }
            if prev_diff.is_some() {
                if prev_diff.unwrap() > 0 && diff < 0 || prev_diff.unwrap() < 0 && diff > 0 {
                    return Some(i);
                }
            }
            prev_diff = Some(diff);
        }
        prev_num = Some(*num);
    }
    return None;
}

fn main() -> io::Result<()> {
    let start_time = Instant::now();
    let file = File::open("src/day02/input.txt")?;
    let reader = BufReader::new(file);

    let mut reports: Vec<Vec<i64>> = Vec::new();
    {
        for line in reader.lines() {
            let mut tmp: Vec<i64> = Vec::new();
            let var_name = line?;
            let parts = var_name.split(" ");
            for part in parts {
                tmp.push(part.parse::<i64>().unwrap());
            }

            reports.push(tmp);
        }
    }

    let mut safe: i64 = 0;
    let mut fixable: i64 = 0;

    for numbers in reports {
        let result = check(&numbers);
        if result.is_none() {
            safe += 1;
        } else {
            for i in 0..numbers.len() {
                let mut cpy = numbers.clone();
                cpy.remove(i);
                if check(&cpy).is_none() {
                    fixable += 1;
                    break;
                }
            }
        }
    }
    println!("Part1: {}", safe);
    println!("Part2: {}", safe + fixable);
    let elapsed = start_time.elapsed();
    println!("Run time: {:?}", elapsed);

    Ok(())
}
