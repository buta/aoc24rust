use std::collections::HashSet;
use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::time::Instant;

use aoc24rust::utils::{PointT, RectT};

type Map = Vec<Vec<Option<char>>>;
type Point = PointT<i64>;
type Rect = RectT<i64>;

const DIRECTIONS: [Point; 4] = [
    Point { x: 0, y: -1 },
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: -1, y: 0 },
];
const OPERATIONS: [fn(i64, i64) -> i64; 2] = [|a, b| a + b, |a, b| a - b];

struct Area {
    id: char,
    area: i64,
    fences: HashSet<Fence>,
}
#[derive(PartialEq, Eq, Hash, Clone)]
struct Fence {
    inside: Point,
    outside: Point,
}

struct Garden {
    map: Map,
    bounds: Rect,
    areas: Vec<Area>,
}

impl Display for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: size={} fences={}",
            self.id,
            self.area,
            self.fences.len()
        )
    }
}
impl Fence {
    fn new(in_: Point, out: Point) -> Fence {
        assert!(in_.distance(&out) == 1, "Invalid coordinates");
        Fence {
            inside: in_,
            outside: out,
        }
    }
}
impl Garden {
    fn parse(input: &str) -> Garden {
        let mut map: Map = Vec::new();
        for line in input.lines() {
            map.push(line.chars().map(|v| Some(v)).collect::<Vec<Option<char>>>());
        }
        let bounds = Rect {
            x: 0,
            y: 0,
            width: map[0].len() as i64,
            height: map.len() as i64,
        };
        return Garden {
            map: map,
            bounds: bounds,
            areas: Vec::new(),
        };
    }

    fn rec_map_area(
        &self,
        id: char,
        p: Point,
        prev: Option<&Point>,
        out_visited: &mut HashSet<Point>,
        out_fences: &mut HashSet<Fence>,
    ) -> i64 {
        let mut size: i64 = 0;
        if !out_visited.contains(&p) {
            if self.bounds.contains_point(&p) && self.map[p.y as usize][p.x as usize] == Some(id) {
                out_visited.insert(p.clone());
                size += 1;
                for dir in DIRECTIONS {
                    let target = p.clone() + dir;
                    size += self.rec_map_area(id, target, Some(&p), out_visited, out_fences)
                }
            } else if let Some(prev) = prev {
                out_fences.insert(Fence::new(prev.clone(), p));
            }
        }
        return size;
    }

    fn build_areas(self: &mut Self) {
        self.areas.clear();
        for y in 0..self.bounds.height {
            for x in 0..self.bounds.width {
                if let Some(chr) = self.map[y as usize][x as usize] {
                    let mut visited: HashSet<Point> = HashSet::new();
                    let mut fences: HashSet<Fence> = HashSet::new();
                    let size = self.rec_map_area(
                        chr,
                        Point { x: x, y: y },
                        None,
                        &mut visited,
                        &mut fences,
                    );
                    // remove used chars from the map
                    for point in visited {
                        if self.map[point.y as usize][point.x as usize] == Some(chr) {
                            self.map[point.y as usize][point.x as usize] = None;
                        }
                    }
                    let area = Area {
                        id: chr,
                        area: size,
                        fences: fences,
                    };
                    self.areas.push(area);
                }
            }
        }
    }

    fn get_part1(&self) -> i64 {
        self.areas
            .iter()
            .fold(0, |acc, a| acc + a.area * a.fences.len() as i64)
    }

    fn get_part2(&self) -> i64 {
        let mut ret: i64 = 0;
        for area in self.areas.iter() {
            let mut used: HashSet<Fence> = HashSet::new();
            let mut num_fences = 0;
            for fence in area.fences.iter() {
                if !used.contains(fence) {
                    num_fences += 1;
                    let dx = match fence.inside.x == fence.outside.x {
                        true => 1_i64,
                        false => 0_i64,
                    };
                    let dy = match fence.inside.y == fence.outside.y {
                        true => 1_i64,
                        false => 0_i64,
                    };
                    assert!(dx + dy == 1, "Invalid fence");
                    for op in OPERATIONS {
                        for i in 1_i64.. {
                            let neighbour = Fence::new(
                                Point {
                                    x: op(fence.inside.x, dx * i),
                                    y: op(fence.inside.y, dy * i),
                                },
                                Point {
                                    x: op(fence.outside.x, dx * i),
                                    y: op(fence.outside.y, dy * i),
                                },
                            );
                            if area.fences.contains(&neighbour) {
                                used.insert(neighbour);
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
            ret += area.area * num_fences;
        }
        return ret;
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    let input = fs::read_to_string("src/day12/input.txt")?;
    let mut garden = Garden::parse(&input);
    garden.build_areas();

    println!("Part1: {}", garden.get_part1());
    println!("Part2: {}", garden.get_part2());
    let elapsed = start_time.elapsed();
    println!("Run time: {:?}", elapsed);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_small_examples() {
        {
            let mut garden = Garden::parse(
                "AAAA
BBCD
BBCC
EEEC",
            );
            garden.build_areas();
            assert!(garden.get_part1() == 140);
            assert!(garden.get_part2() == 80);
        }
    }

    #[test]
    fn test_fully_contained() {
        {
            let mut garden = Garden::parse(
                "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO",
            );
            garden.build_areas();
            assert!(garden.get_part1() == 772);
            assert!(garden.get_part2() == 436);
        }
    }

    #[test]
    fn test_mobius_fencing() {
        {
            let mut garden = Garden::parse(
                "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA",
            );
            garden.build_areas();
            assert!(garden.get_part2() == 368);
        }
    }

    #[test]
    fn test_dbg() {
        let input = fs::read_to_string("src/day12/dbg.txt").unwrap();
        let mut garden = Garden::parse(&input);
        garden.build_areas();
        assert!(garden.get_part1() == 1930);
        assert!(garden.get_part2() == 1206);
    }
}
