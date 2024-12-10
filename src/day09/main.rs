use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::time::Instant;

struct File {
    id: i32,
    size: usize,
    initial_index: usize,
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id={} size={}", self.id, self.size)
    }
}

fn checksum(disk: &Vec<Option<&File>>) -> i64 {
    let mut ret: i64 = 0;
    for (i, slot) in disk.iter().enumerate() {
        match slot {
            Some(file) => ret += i as i64 * file.id as i64,
            None => {}
        }
    }
    return ret;
}

#[allow(dead_code)]
fn draw(disk: &Vec<Option<&File>>) {
    let mut tmp = String::new();
    for i in disk.iter() {
        match i {
            Some(file) => tmp.push_str(file.id.to_string().as_str()),
            None => tmp.push('.'),
        }
    }
    println!("{}", tmp);
}

fn part1(mut disk: Vec<Option<&File>>) -> i64 {
    let mut front_idx: usize = 0;
    let mut rear_idx = disk.len() - 1;
    while front_idx < rear_idx {
        let file: Option<&File> = disk[rear_idx];
        if file.is_none() {
            rear_idx -= 1;
            continue;
        }
        let file = file.unwrap();
        for i in 0..file.size {
            while front_idx < rear_idx - i {
                if disk[front_idx].is_none() {
                    break;
                } else {
                    front_idx += 1
                }
            }
            if disk[front_idx].is_some() {
                break;
            }
            disk[front_idx] = disk[rear_idx - i];
            disk[rear_idx - i] = None;
            front_idx += 1;
        }
    }
    return checksum(&disk);
}

fn find_space(
    find_start: &mut usize,
    disk: &Vec<Option<&File>>,
    size: usize,
    end: usize,
) -> Option<usize> {
    let mut streak = 0;
    let mut only_used = true;
    for i in *find_start..end {
        if disk[i].is_some() {
            if only_used {
                *find_start = i;
            } else {
                streak = 0;
            }
        } else {
            streak += 1;
            only_used = false;
            if streak == size {
                return Some(i - (size - 1));
            }
        }
    }
    return None;
}

fn part2(mut disk: Vec<Option<&File>>, files: &Vec<File>) -> i64 {
    let mut find_start: usize = 0;

    for file in files.iter().rev() {
        let idx = find_space(&mut find_start, &disk, file.size, file.initial_index);
        match idx {
            Some(idx) => {
                for i in 0..file.size {
                    assert!(disk[idx + i].is_none());
                    disk[idx + i] = disk[file.initial_index + i];
                    disk[file.initial_index + i] = None;
                }
            }
            None => {}
        }
    }
    return checksum(&disk);
}
fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    let mut files: Vec<File> = Vec::new();

    let mut disk: Vec<Option<&File>> = Vec::new();
    let input: String = fs::read_to_string("src/day09/input.txt")?;
    let mut position: usize = 0;
    for (i, c) in input.chars().enumerate() {
        if (i & 1) == 0 {
            {
                let file = File {
                    id: (i / 2) as i32,
                    size: c.to_digit(10).expect("Invalid number") as usize,
                    initial_index: position,
                };
                position += file.size;
                files.push(file);
            }
        } else {
            position += c.to_digit(10).expect("Invalid number") as usize;
        }
    }
    let last = files.last().unwrap();
    disk.resize(last.initial_index + last.size, None);
    for file in files.iter() {
        for i in file.initial_index..file.initial_index + file.size {
            disk[i] = Some(file);
        }
    }

    let part1: i64 = part1(disk.clone());
    let part2: i64 = part2(disk, &files);

    let elapsed = start_time.elapsed();
    println!("Part1: {}\nPart2: {}", part1, part2);
    println!("Run time: {:?}", elapsed);
    Ok(())
}
