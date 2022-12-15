use std::collections::HashSet;
use std::error::Error;
use std::fs;

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
enum Dir {
    U,
    D,
    L,
    R,
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
    dir: Dir, // Only for moves, combining data structures for puzzle convenience
}

impl Point {
    fn new(x: i32, y: i32, dir: Dir) -> Self {
        Point { x, y, dir }
    }

    fn check_and_mv(&mut self, head: &Point) -> bool {
        let delta = Point::new(head.x - self.x, head.y - self.y, head.dir);

        if delta.x.abs() > 1 || delta.y.abs() > 1 {
            if delta.x.abs().is_positive() {
                self.x += delta.x.signum();
            }
            if delta.y.abs().is_positive() {
                self.y += delta.y.signum();
            }
            return true;
        }

        return false;
    }

    // Step by one
    fn step(&mut self, dir: Dir) {
        match dir {
            Dir::R => {
                self.x += 1;
            }
            Dir::L => {
                self.x -= 1;
            }
            Dir::U => {
                self.y += 1;
            }
            Dir::D => {
                self.y -= 1;
            }
        }
    }
}

// U 10, R 3 to direction and coordinates
fn dir_to_point(dir: &str) -> Point {
    let tmp: Vec<&str> = dir.split(" ").collect::<Vec<&str>>();

    match tmp[0] {
        "U" => Point::new(0, tmp[1].parse::<i32>().unwrap(), Dir::U),
        "D" => Point::new(0, -tmp[1].parse::<i32>().unwrap(), Dir::D),
        "L" => Point::new(-tmp[1].parse::<i32>().unwrap(), 0, Dir::L),
        "R" => Point::new(tmp[1].parse::<i32>().unwrap(), 0, Dir::R),
        _ => {
            panic!("!!! dir_to_point");
        }
    }
}

// nknots counts the head as one of the knots, so nknots=10 would have 9 tail knots
// Return set of all places tail has seen, head point, tail points
fn perform_moves_n(moves: Vec<Point>, nknots: usize) -> (HashSet<Point>, Point, Vec<Point>) {
    let mut head = Point::new(0, 0, Dir::L);
    let mut seen: HashSet<Point> = HashSet::new();
    seen.insert(Point::new(0, 0, Dir::L));

    let mut knots: Vec<Point> = Vec::new();
    for _ in 0..nknots {
        knots.push(Point::new(0, 0, Dir::L));
    }

    for mv in moves {
        for _ in 0..(mv.x.abs() + mv.y.abs()) {
            head.step(mv.dir);

            #[allow(unused)]
            let mut moved = false;

            for k in (0..knots.len()).rev() {
                if k < knots.len() - 1 {
                    let np: Point = knots[k + 1].clone();
                    moved = knots[k].check_and_mv(&np);
                } else {
                    moved = knots[k].check_and_mv(&head);
                }
                if moved && k == 0 {
                    seen.insert(Point::new(knots[0].x, knots[0].y, Dir::L));
                }
            }
        }
    }

    (seen, head, knots)
}

fn moves_from_vec(lines: Vec<&str>) -> Vec<Point> {
    lines
        .iter()
        .map(|l| dir_to_point(&l))
        .collect::<Vec<Point>>()
}

fn render_seen(seen: &HashSet<Point>, dim: (i32, i32, i32, i32)) -> String {
    let mut s: String = String::new();
    for y in (dim.2..(dim.3 + 1)).rev() {
        for x in dim.0..(dim.1 + 1) {
            if seen.get(&Point::new(x, y, Dir::L)).is_some() {
                s.push('#');
            } else {
                s.push('.')
            }
        }
        s.push('\n');
    }
    s
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("src/puzzle.txt")?;
    let lines: Vec<&str> = contents.split("\n").collect::<Vec<&str>>();

    let moves = moves_from_vec(lines);
    let (seen, _, _) = perform_moves_n(moves.clone(), 1);
    eprintln!("seen = {:?}", seen.len());
    let (seen, _, _) = perform_moves_n(moves, 9);
    eprintln!("seen = {:?}", seen.len());

    let minx = seen.iter().map(|p| p.x).min().unwrap_or(0);
    let maxx = seen.iter().map(|p| p.x).max().unwrap_or(0);
    let miny = seen.iter().map(|p| p.y).min().unwrap_or(0);
    let maxy = seen.iter().map(|p| p.y).max().unwrap_or(0);

    // print!("{}", render_seen(&seen, (minx, maxx, miny, maxy)));

    Ok(())
}
