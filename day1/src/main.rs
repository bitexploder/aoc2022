use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("src/puzzle.txt")?;

    let mut elves: Vec<u32> = contents
        .split("\n\n")
        .map(|elf| {
            // Take each elf backpack, split, parse, and sum it
            elf.split("\n")
                .map(|s| s.parse().unwrap_or(0))
                .collect::<Vec<u32>>()
                .iter()
                .sum()
        })
        .collect();

    elves.sort();

    let top3 = &elves[elves.len() - 3..];

    println!("Top 3: {:?}", top3[0] + top3[1] + top3[2]);

    Ok(())
}
