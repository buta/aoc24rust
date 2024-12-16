use std::{error::Error, fmt::Debug, fs, time::Instant};

use aoc24rust::utils::PointT;

type Point = PointT<i64>;
struct Machine {
    delta_a: Point,
    delta_b: Point,
    prize: Point,
}
impl Debug for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let p1 = self.solve_p1();
        let p2 = self.solve_p2();
        write!(
            f,
            "A:{}, B:{} Prize:{} Part1={:?} Part2={:?}",
            self.delta_a, self.delta_b, self.prize, p1, p2
        )
    }
}
impl Machine {
    fn parse(lines: &str) -> Option<Machine> {
        let mut d_a: Option<Point> = None;
        let mut d_b: Option<Point> = None;
        let mut p: Option<Point> = None;

        for line in lines.lines() {
            let parse_int_pair = |value: &str| -> Option<Point> {
                if let [x, y] = value
                    .split(',')
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|v| {
                        v.chars()
                            .map(|c| c.to_digit(10))
                            .flatten()
                            .fold(0_i64, |acc, digit| acc * 10 + digit as i64)
                    })
                    .collect::<Vec<i64>>()[..]
                {
                    return Some(Point { x, y });
                }
                None
            };
            let parts: Vec<&str> = line.split(':').collect();
            match parts[..] {
                ["Button A", value] => d_a = parse_int_pair(value),
                ["Button B", value] => d_b = parse_int_pair(value),
                ["Prize", value] => p = parse_int_pair(value),
                _ => {}
            };
        }
        return match (d_a, d_b, p) {
            (Some(d_a), Some(d_b), Some(p)) => Some(Machine {
                delta_a: d_a,
                delta_b: d_b,
                prize: p,
            }),
            _ => None,
        };
    }

    fn solve_p1(&self) -> Option<i64> {
        let a = self.delta_a.x;
        let b = self.delta_b.x;
        let c = self.prize.x;
        let d = self.delta_a.y;
        let e = self.delta_b.y;
        let f = self.prize.y;
        let top = a * f - d * c;
        let bottom = a * e - d * b;
        if top % bottom != 0 {
            return None;
        }
        let y = top / bottom;
        let top = c - b * y;
        if top % a != 0 {
            return None;
        }
        let x = top / a;
        if x > 100 || y > 100 {
            return None;
        }
        return Some(x * 3 + y);
    }

    fn solve_p2(&self) -> Option<i128> {
        const ERROR: i128 = 10000000000000;
        let a = self.delta_a.x as i128;
        let b = self.delta_b.x as i128;
        let c = self.prize.x as i128 + ERROR;
        let d = self.delta_a.y as i128;
        let e = self.delta_b.y as i128;
        let f = self.prize.y as i128 + ERROR;
        let top = a * f - d * c;
        let bottom = a * e - d * b;
        if top % bottom != 0 {
            return None;
        }
        let y = top / bottom;
        let top = c - b * y;
        if top % a != 0 {
            return None;
        }
        let x = top / a;
        return Some(x * 3 + y);
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    let input = fs::read_to_string("src/day13/input.txt")?.replace("\r\n", "\n");
    let machines = input
        .split("\n\n")
        .into_iter()
        .map(|line| Machine::parse(line))
        .flatten()
        .collect::<Vec<Machine>>();
    let part1 = machines
        .iter()
        .map(|m| m.solve_p1())
        .collect::<Vec<Option<i64>>>()
        .iter()
        .flatten()
        .sum::<i64>();

    let part2 = machines
        .iter()
        .map(|m| m.solve_p2())
        .collect::<Vec<Option<i128>>>()
        .iter()
        .flatten()
        .sum::<i128>();

    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
    let elapsed = start_time.elapsed();
    println!("Run time: {:?}", elapsed);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_small_example() {
        let input = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;
        let machines = input
            .split("\n\n")
            .into_iter()
            .map(|line| Machine::parse(line))
            .flatten()
            .collect::<Vec<Machine>>();
        let solutions = machines
            .iter()
            .map(|m| m.solve_p1())
            .collect::<Vec<Option<i64>>>();
        let sum = solutions.iter().flatten().sum::<i64>();
        assert!(sum == 480);
        let solutions2 = machines
            .iter()
            .map(|m| m.solve_p2())
            .collect::<Vec<Option<i128>>>();
        let sum2 = solutions2.iter().flatten().sum::<i128>();
        assert!(sum2 == 875318608908);
    }
}
