use itertools::Itertools;
use std::error::Error;
use std::fs;
use take_until::TakeUntilExt;

const TESTGRID: &str = &"30373
25512
65332
33549
35390";

fn parse_grid(gstr: &str) -> Vec<Vec<u32>> {
    let lines = gstr.split("\n").collect::<Vec<&str>>();
    let grid: Vec<Vec<u32>> = Vec::new();
    lines
        .iter()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

fn vec_vis(tree: &u32, tree_row: &[u32], rev: bool) -> bool {
    if rev {
        tree_row.iter().rev().fold(true, |acc, some_tree| {
            if some_tree < &tree && acc {
                return true;
            }
            false
        })
    } else {
        tree_row.iter().fold(true, |acc, some_tree| {
            if some_tree < &tree && acc {
                return true;
            }
            false
        })
    }
}

// Only run this if view corridor is good
fn vec_vis_cnt(tree: &u32, tree_row: &[u32], rev: bool) -> u32 {
    let mut cnt = 0;

    if rev {
        cnt = tree_row
            .iter()
            .rev()
            .take_until(|z| *z >= &tree)
            .fold(0, |acc, z| acc + 1);
    } else {
        cnt = tree_row
            .iter()
            .take_until(|z| *z >= &tree)
            .fold(0, |acc, z| acc + 1);
    }

    cnt
}

fn get_scenic_score(x: usize, y: usize, grid: &Vec<Vec<u32>>) -> u32 {
    let cur_tree = grid[y][x];
    let r = &grid[y][x + 1..];
    let l = &grid[y][..x];

    let mut col: Vec<u32> = Vec::new();
    for i in 0..grid[x].len() {
        col.push(grid[i][x]);
    }
    let u = &col[..y];
    let d = &col[y + 1..];

    let rscore = vec_vis_cnt(&cur_tree, r, false);
    let lscore = vec_vis_cnt(&cur_tree, l, true);
    let dscore = vec_vis_cnt(&cur_tree, d, false);
    let uscore = vec_vis_cnt(&cur_tree, u, true);

    // println!(
    //     "[{}][{}]={}, [u:{},d:{},l:{},r:{}]",
    //     y, x, &cur_tree, &uscore, &dscore, &lscore, &rscore
    // );

    return rscore * lscore * dscore * uscore;
}

fn test_vis(x: usize, y: usize, grid: &Vec<Vec<u32>>) -> bool {
    let cur_tree = grid[y][x];
    let r = &grid[y][x + 1..];
    let l = &grid[y][..x];

    let mut col: Vec<u32> = Vec::new();
    for i in 0..grid[x].len() {
        col.push(grid[i][x]);
    }
    let u = &col[..y];
    let d = &col[y + 1..];

    let rtrue = vec_vis(&cur_tree, r, false);
    let ltrue = vec_vis(&cur_tree, l, true);
    let dtrue = vec_vis(&cur_tree, d, false);
    let utrue = vec_vis(&cur_tree, u, true);

    if ltrue || rtrue || dtrue || utrue {
        return true;
    }

    false
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("src/puzzle.txt")?;
    let grid = parse_grid(contents.as_str());

    let grid_dim = grid[0].len();
    let edge_cnt = grid_dim * 2 + (grid_dim - 2) * 2;
    let mut vis_cnt: u32 = 0;
    let mut scenic: Vec<u32> = Vec::new();

    for y in 1..grid[0].len() - 1 {
        for x in 1..grid[y].len() - 1 {
            // eprintln!("grid[{}][{}] = {:?}", y, x, grid[y][x]);

            // Part 1
            if test_vis(x, y, &grid) {
                vis_cnt += 1;
            }

            // Part 2
            scenic.push(get_scenic_score(x, y, &grid));
        }
        // println!("{}-----------", vis_cnt);
    }

    println!("vis_cnt: {}", vis_cnt);
    println!("total_cnt: {}", edge_cnt + vis_cnt as usize);
    println!("highest_scenic: {:?}", scenic.iter().max().unwrap());

    Ok(())
}
