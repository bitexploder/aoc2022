use pathfinding::prelude::bfs;
use std::error::Error;
use std::fs;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Coord { x, y }
    }

    fn neighbors(&self, terrain: &[Vec<u8>]) -> Vec<Coord> {
        get_neighbors(terrain, self.x, self.y)
    }
}

fn find_path(terrain: &[Vec<u8>], start: Coord, end: Coord) -> usize {
    let path = bfs(&start, |n| n.neighbors(terrain), |n| *n == end);
    path.unwrap_or_default().len()
}

fn parse_terrain(contents: String) -> Vec<Vec<u8>> {
    contents
        .split("\n")
        .map(|l| l.chars().map(|c| c as u8).collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>()
}

fn get_height(terrain: &[Vec<u8>], x: usize, y: usize) -> (Coord, u8) {
    (Coord::new(x, y), terrain[y][x])
}

fn valid_move(cur: &u8, new: &u8) -> bool {
    match (cur, new) {
        (b'z' | b'y', b'E') => true,
        (b'S', b'a') => true,
        (cur, new) if cur + 1 >= *new => true,
        _ => false,
    }
}

fn get_neighbors(terrain: &[Vec<u8>], x: usize, y: usize) -> Vec<Coord> {
    let mut neighbors: Vec<(Coord, u8)> = Vec::new();
    let cur_height = terrain[y][x];

    if y > 0 {
        neighbors.push(get_height(&terrain, x, y - 1)); // up
    }
    if y < terrain.len() - 1 {
        neighbors.push(get_height(&terrain, x, y + 1)); // down
    }
    if x > 0 {
        neighbors.push(get_height(&terrain, x - 1, y)) // left
    }
    if x < terrain[y].len() - 1 {
        neighbors.push(get_height(&terrain, x + 1, y)); // right
    }

    // Filter invalid neighbors
    neighbors
        .iter()
        .filter(|n| valid_move(&cur_height, &n.1))
        .map(|n| n.0.clone())
        .collect::<Vec<Coord>>()
}

fn find_u8(terrain: &[Vec<u8>], needle: u8) -> Vec<Coord> {
    let mut found: Vec<Coord> = Vec::new();
    for (y, _) in terrain.iter().enumerate() {
        for (x, hay) in terrain[y].iter().enumerate() {
            if hay == &needle {
                found.push(Coord::new(x, y));
            }
        }
    }
    found
}

fn part1(terrain: &[Vec<u8>]) {
    let start = find_u8(&terrain, b'S')[0];
    let end = find_u8(&terrain, b'E')[0];
    eprintln!("start = {:?}", start);
    eprintln!("end = {:?}", end);
    let steps = find_path(&terrain, start, end);
    eprintln!("steps = {:?}", steps - 1);
}

fn part2(terrain: &[Vec<u8>]) {
    let end = find_u8(terrain, b'E')[0];
    let starts = find_u8(terrain, b'a');
    let paths = starts
        .iter()
        .map(|start| find_path(&terrain, *start, end))
        .filter(|path| path > &0)
        .collect::<Vec<usize>>();
    eprintln!("min = {:?}", paths.iter().min().unwrap() - &1);
}

// S = 83, E = 69, nice
fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("src/puzzle.txt")?;
    let terrain = parse_terrain(contents);
    part1(&terrain);
    part2(&terrain);

    Ok(())
}
