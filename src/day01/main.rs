use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::time::Instant;

fn main() -> io::Result<()> {
    let start_time = Instant::now();
    let file = File::open("src/day01/input01.txt")?;
    let reader = BufReader::new(file);

    let mut first: Vec<i64> = Vec::new();
    let mut second: Vec<i64> = Vec::new();
    {
        //let re = Regex::new(r"^(?<first>\d+)\s+(?<second>\d+)").unwrap();
        for line in reader.lines() {
            let tmp = line?;
            let idx = tmp.find(' ').unwrap();
            let last_idx = tmp.rfind(' ').unwrap();
            let a = tmp[..idx].parse::<i64>().unwrap();
            let b = tmp[last_idx + 1..].parse::<i64>().unwrap();
            first.push(a);
            second.push(b);
        }
    }

    first.sort();
    second.sort();
    {
        let mut distance: i64 = 0;
        for (a, b) in first.iter().zip(second.iter()) {
            distance += (a - b).abs();
        }
        println!("Part1: {}", distance);
    }
    {
        let mut similarity: i64 = 0;
        for a in first.iter() {
            let found: i64 = second.iter().filter(|&n| n == a).count() as i64;
            similarity += a * found;
        }

        println!("Part2: {}", similarity);
        let elapsed = start_time.elapsed();
        println!("Run time: {:?}", elapsed);
    }
    Ok(())
}
