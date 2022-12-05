use itertools::Itertools;
use std::error::Error;
use std::fs;

fn get_sacks(contents: &String) -> Vec<&str> {
    contents.split("\n").collect()
}

fn get_sacks_with_compartments(contents: &String) -> Vec<(&str, &str)> {
    contents
        .split("\n")
        .map(|s|
            //(s, s),
            (&s[..s.len()/2], &s[s.len()/2..]))
        .collect()
}

fn get_sack_prio(sack: &(&str, &str)) -> u32 {
    for c in sack.0.chars() {
        if sack.1.contains(c) {
            return prio(c);
        }
    }

    0
}

fn get_all_sack_prio(sacks: &Vec<(&str, &str)>) -> Vec<u32> {
    let mut common_prio: Vec<u32> = Vec::new();

    for sack in sacks {
        let sack_prio = get_sack_prio(sack);
        common_prio.push(sack_prio);

        // Just some happy little sanity checks
        assert_ne!(sack_prio, 0);
        assert_eq!(sack.0.len(), sack.1.len())
    }

    common_prio
}

fn prio(c: char) -> u32 {
    if c >= 'a' && c <= 'z' {
        return c as u32 - 96;
    }
    if c >= 'A' && c <= 'Z' {
        return (c as u32 - 65) + 27;
    }

    0
}

#[test]
fn test_prio() {
    assert_eq!(1, prio('a'));
    assert_eq!(26, prio('z'));
    assert_eq!(27, prio('A'));
    assert_eq!(52, prio('Z'));
}

const TESTCONTENTS: &'static str = &"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

#[test]
fn test_common_prio_part1() {
    let contents = &TESTCONTENTS.to_string();
    let sacks = get_sacks_with_compartments(contents);
    let common_prio = get_all_sack_prio(&sacks);
    let common_prio_sum = common_prio.iter().sum::<u32>();

    assert_eq!(157, common_prio_sum);
}

fn group_elves(sacks: Vec<&str>) -> Vec<Vec<&str>> {
    let chunks = sacks.chunks(3);
    let mut groups: Vec<Vec<&str>> = Vec::new();
    for chunk in chunks {
        groups.push(chunk.iter().map(|s| *s).collect::<Vec<&str>>());
    }
    return groups;
}

fn get_groups_prio(groups: Vec<Vec<&str>>) -> Vec<u32> {
    groups
        .iter()
        .map(|group| {
            // Convert group of elves to a tuple
            let g: (&str, &str, &str) = group.iter().map(|s| *s).collect_tuple().unwrap();
            for c in g.0.chars() {
                if g.1.contains(c) && g.2.contains(c) {
                    return prio(c);
                }
            }
            0
        })
        .collect()
}

#[test]
fn test_find_common_part2() {
    let contents = &TESTCONTENTS.to_string();
    let sacks = get_sacks(&contents);
    let groups_prio = get_groups_prio(group_elves(sacks));
    let sum = groups_prio.iter().sum::<u32>();

    assert_eq!(70, sum);
}

fn part1(contents: &String) -> Result<u32, Box<dyn Error>> {
    let sacks = get_sacks_with_compartments(&contents);
    let common_prio = get_all_sack_prio(&sacks);
    Ok(common_prio.iter().sum::<u32>())
}

fn part2(contents: &String) -> Result<u32, Box<dyn Error>> {
    let sacks = get_sacks(&contents);
    let groups_prio = get_groups_prio(group_elves(sacks));
    Ok(groups_prio.iter().sum::<u32>())
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("src/puzzle.txt")?;

    println!("part1: {:?}", part1(&contents));
    println!("part2: {:?}", part2(&contents));

    Ok(())
}
