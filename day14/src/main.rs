use std::error::Error;
use std::fs;

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(s: &str) -> Self {
        let parts = s
            .split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|part| part.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        return Coord {
            x: parts[0],
            y: parts[1],
        };
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Space {
    Air,
    Sand,
    Rock,
}

fn place_rocks(cave: &mut Vec<Vec<Space>>, rocks: Vec<Vec<Coord>>) {
    let mut turtle = Coord { x: 0, y: 0 };

    for chain in rocks {
        // Start here
        turtle.x = chain[0].x;
        turtle.y = chain[0].y;

        for inst in &chain[1..] {
            let mut r = (0..=0);

            if turtle.x == inst.x {
                // go down
                if turtle.y < inst.y {
                    r = turtle.y..=inst.y;
                }
                // go up
                if turtle.y > inst.y {
                    r = inst.y..=turtle.y;
                }
                for d in r {
                    cave[turtle.x][d] = Space::Rock;
                }
                turtle.y = inst.y;
            } else if turtle.y == inst.y {
                if turtle.x < inst.x {
                    r = turtle.x..=inst.x;
                }
                if turtle.x > inst.x {
                    r = inst.x..=turtle.x;
                }
                for d in r {
                    cave[d][turtle.y] = Space::Rock;
                }
                turtle.x = inst.x;
            }
        }
    }
}

fn render_cave(cave: &Vec<Vec<Space>>, size: (usize, usize, usize, usize)) {
    let mut lr: String = String::from("");
    for i in size.1..size.3 {
        if i != 500 {
            lr.push('.');
        } else {
            lr.push('+');
        }
    }
    println!("y: ZZZ: {}", lr);
    for y in size.1..size.3 {
        lr = String::from("");

        for i in size.0..size.2 {
            if cave[i][y] == Space::Air {
                lr.push('.');
            } else if cave[i][y] == Space::Sand {
                lr.push('o');
            } else {
                lr.push('#');
            }
        }
        println!("y: {:03}: {}", y, lr);
    }
}

fn drop_grain(cave: &mut Vec<Vec<Space>>) -> bool {
    // println!("== begin drop ==");
    let mut grain = Coord { x: 500, y: 0 };
    let mut below = Space::Air;
    let mut below_left = Space::Air;
    let mut below_right = Space::Air;

    let mut cnt = 0;

    if cave[grain.x][grain.y] == Space::Sand {
        return false;
    }

    loop {
        below_left = cave[grain.x - 1][grain.y + 1];
        below = cave[grain.x][grain.y + 1];
        below_right = cave[grain.x + 1][grain.y + 1];

        let blocked = (
            (below_left == Space::Rock || below_left == Space::Sand),
            (below == Space::Rock || below == Space::Sand),
            (below_right == Space::Rock || below_right == Space::Sand),
        );

        match blocked {
            (true, true, true) => {
                cave[grain.x][grain.y] = Space::Sand;
                break;
            }

            (false, true, false) => {
                grain.x -= 1;
            }

            (true, true, false) => {
                grain.x += 1;
            }

            (false, true, true) => {
                grain.x -= 1;
            }

            _ => {}
        }
        grain.y += 1;

        if cnt > 164 {
            return false;
        }
        cnt += 1;
    }

    true
}

fn place_floor(cave: &mut Vec<Vec<Space>>) {
    // Part 2
    let mut max_y = 0;
    for x in 0..cave.len() - 1 {
        for y in 0..cave.len() - 1 {
            if cave[x][y] == Space::Rock {
                if y > max_y {
                    max_y = y;
                }
            }
        }
    }

    let floor = max_y + 2;
    for x in 0..cave.len() - 1 {
        cave[x][floor] = Space::Rock;
    }
    // End part 2
}
fn main() -> std::result::Result<(), Box<dyn Error>> {
    // let contents = fs::read_to_string("src/test.txt")?;
    let contents = fs::read_to_string("src/puzzle.txt")?;

    // Collect some rock instructions
    let rocks = contents
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| {
            line.split(" -> ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|rock_str| Coord::new(rock_str))
                .collect::<Vec<Coord>>()
        })
        .collect::<Vec<Vec<Coord>>>();

    let mut cave = vec![vec![Space::Air; 2000]; 2000];
    // let mut cave = vec![vec![Space::Air; 1000]; 1000];

    place_rocks(&mut cave, rocks);

    // part 2, comment out for part 1
    place_floor(&mut cave);

    let mut grains_dropped = 0;
    loop {
        if !drop_grain(&mut cave) {
            break;
        }
        grains_dropped += 1;
    }

    // Testing
    // render_cave(&cave, (488, 0, 516, 14));
    // render_cave(&cave, (495, 25, 525, 36));
    // Puzzle.txt full
    // render_cave(&cave, (300, 0, 700, 163));
    println!("Dropped {} grains", grains_dropped);
    Ok(())
}
