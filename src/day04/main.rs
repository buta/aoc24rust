use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::time::Instant;

const WORD: &str = "XMAS";
const DIRECTIONS: &'static [(i64, i64)] = &[
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    let mut data = String::new();
    let mut x_max: usize = 0;
    let mut y_max: usize = 0;
    let file = File::open("src/day04/input.txt")?;
    let reader = BufReader::new(file);

    for line_rs in reader.lines() {
        let line = line_rs?;
        let line_len = line.len();
        if x_max == 0 {
            x_max = line_len;
        } else if line_len > 0 && line_len != x_max {
            panic!("Input width is not uniform!");
        }
        data.push_str(&line);
        y_max += 1;
    }

    let check_char = |x: i64, y: i64, chr: Option<char>| -> bool {
        return x >= 0
            && y >= 0
            && x < x_max as i64
            && y < y_max as i64
            && data.chars().nth(x as usize + y as usize * x_max) == chr;
    };

    let check_word = |x: i64, y: i64| -> i64 {
        let mut ret = 0;
        'dirs: for (dx, dy) in DIRECTIONS {
            for i in 0..WORD.len() as i64 {
                if !check_char(x + dx * i, y + dy * i, WORD.chars().nth(i as usize)) {
                    continue 'dirs;
                }
            }
            ret += 1;
        }
        return ret;
    };
    let check_mas = |x: i64, y: i64| -> bool {
        return check_char(x, y, Some('A'))
            && (check_char(x + 1, y + 1, Some('M')) && check_char(x - 1, y - 1, Some('S'))
                || check_char(x + 1, y + 1, Some('S')) && check_char(x - 1, y - 1, Some('M')))
            && (check_char(x + 1, y - 1, Some('M')) && check_char(x - 1, y + 1, Some('S'))
                || check_char(x + 1, y - 1, Some('S')) && check_char(x - 1, y + 1, Some('M')));
    };
    let mut part1: i64 = 0;
    let mut part2: i64 = 0;
    for y in 0..y_max as i64 {
        for x in 0..x_max as i64 {
            part1 += check_word(x, y);
            if check_mas(x, y) {
                part2 += 1;
            }
        }
    }

    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
    let elapsed = start_time.elapsed();
    println!("Run time: {:?}", elapsed);

    Ok(())
}
