use std::error::Error;
use std::fs;

const TARGET_CYCLES: &[u32] = &[20, 60, 100, 140, 180, 220];

// Part 1
fn check_cycle(ss: &mut Vec<u32>, cycle: u32, reg: i32) {
    if TARGET_CYCLES.contains(&cycle) {
        ss.push(cycle * reg as u32);
    }
}

// Part 2
fn check_cycle2(crt: &mut Vec<String>, cycle: u32, reg: i32) {
    let row: usize = (cycle / 40) as usize;
    let col = (cycle % 40) as usize;

    let sprite: Vec<i32> = (reg - 1..reg + 2)
        .filter(|p| p >= &0 && p < &40)
        .collect::<Vec<i32>>();

    if sprite.contains(&(col as i32)) {
        crt[row].replace_range(col..col + 1, "#");
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("src/puzzle.txt")?;

    let mut reg: i32 = 1;
    let mut cycle: u32 = 0;
    let mut ss: Vec<u32> = Vec::new();
    let mut crt: Vec<String> = vec![".".repeat(40); 6];
    // let mut pixels: String = "".to_string();

    for instruction in contents.split("\n").collect::<Vec<&str>>() {
        match instruction.split(" ").collect::<Vec<&str>>()[..] {
            [_, val] => {
                for i in 0..2 {
                    check_cycle2(&mut crt, cycle, reg);
                    cycle += 1;
                    check_cycle(&mut ss, cycle, reg);
                }
                let v: i32 = val.parse().unwrap();
                reg += v;
            }

            _ => {
                check_cycle2(&mut crt, cycle, reg);
                cycle += 1;
                check_cycle(&mut ss, cycle, reg);
            }
        }
    }

    // Part 1
    println!("part 1: {:?}", ss.iter().sum::<u32>());

    // Part 1
    for row in crt {
        println!("{}", row);
    }
    Ok(())
}
