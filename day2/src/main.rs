use phf::phf_map;
use std::error::Error;
use std::fs;

// A = Rock, B = Paper, C = Scissors
// X = Rock (1), Y = Paper (2), Z = Scissors (3)
const XROCK: u32 = 1;
const YPAPE: u32 = 2;
const ZSCIS: u32 = 3;
const WIN: u32 = 6;
const DRAW: u32 = 3;
const LOSE: u32 = 0;

static RPS: phf::Map<&'static str, u32> = phf_map! {
    "A X" => add(XROCK, DRAW),  // Rock v Rock -- Draw
    "A Y" => add(YPAPE, WIN),   // Rock v Paper -- Win
    "A Z" => add(ZSCIS, LOSE),  // Rock v Scissors -- Lose
    ////////////////////////
    "B X" => add(XROCK, LOSE),  // Paper v Rock -- Lose
    "B Y" => add(YPAPE, DRAW),  // Paper v Paper -- Draw
    "B Z" => add(ZSCIS, WIN),   // Paper v Scissor -- Win
    ////////////////////////
    "C X" => add(XROCK, WIN),   // Scissors v Rock -- Win
    "C Y" => add(YPAPE, LOSE),  // Scissors v Paper -- Lose
    "C Z" => add(ZSCIS, DRAW),  // Scissors v Scissors -- Draw
};

// X = LOSE, Y = DRAW, Z = WIN
// A = Rock, B = Paper, C = Scissors
static RPS2: phf::Map<&'static str, u32> = phf_map! {
    "A X" => add(LOSE, ZSCIS), // Lose to rock, choose scissors
    "A Y" => add(DRAW, XROCK), // Draw to rock, choose rock
    "A Z" => add(WIN, YPAPE),  // Win to rock, choose paper
    ////////////////////////
    "B X" => add(LOSE, XROCK), // Lose to paper, choose rock
    "B Y" => add(DRAW, YPAPE), // Draw to paper, choose paper
    "B Z" => add(WIN, ZSCIS),  // Win to paper, choose scissors
    ////////////////////////
    "C X" => add(LOSE, YPAPE), // Lose to Scissors, choose paper
    "C Y" => add(DRAW, ZSCIS), // Draw to scissors, choose scissors
    "C Z" => add(WIN, XROCK),  // Win to scissors, choose rock
};

const fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("src/puzzle.txt")?;

    let games: Vec<(u32, u32)> = contents
        .split("\n")
        .map(|s| (RPS.get(s).unwrap().clone(), RPS2.get(s).unwrap().clone()))
        .collect();

    println!("Day 2, p1: {:?}", games.iter().map(|t| t.0).sum::<u32>());
    println!("Day 2, p2: {:?}", games.iter().map(|t| t.1).sum::<u32>());

    Ok(())
}
