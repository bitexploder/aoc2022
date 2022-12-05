use itertools::Itertools;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fs;

#[derive(Debug)]
struct Range {
    start: u32,
    end: u32,
    set: HashSet<u32>,
}

impl Range {
    fn parse(s: &str) -> Self {
        let parts = s.split("-").collect::<Vec<&str>>();
        let start: u32 = parts[0].parse().unwrap();
        let end: u32 = parts[1].parse().unwrap();

        // TODO: Make this a nice loop over iter?
        let mut set = HashSet::new();
        for i in start..=end {
            set.insert(i);
        }

        Self { start, end, set }
    }

    fn either_is_subset(&self, r: &Range) -> bool {
        if self.set.is_subset(&r.set) {
            return true;
        }

        if r.set.is_subset(&self.set) {
            return true;
        }

        false
    }

    fn overlaps_with(&self, r: &Range) -> bool {
        let inter = self.set.intersection(&r.set);
        // println!("Intersection: {:?}", inter);

        match inter.count() {
            0 => false,
            _ => true,
        }
    }
}

fn get_pairs(contents: &String) -> Vec<(Range, Range)> {
    contents
        .split("\n")
        .map(|pair| {
            let pairs = pair.split(",").collect::<Vec<&str>>();
            (Range::parse(pairs[0]), Range::parse(pairs[1]))
        })
        .collect()
}

const TESTPAIRS: &str = &"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

#[test]
fn test_set() {
    let pairs = get_pairs(&TESTPAIRS.to_string());

    // println!(
    //     "PAIRS: {:?}",
    //     pairs
    //         .iter()
    //         .map(|pair| (&pair.0.set, &pair.1.set))
    //         .collect::<Vec<(&HashSet<u32>, &HashSet<u32>)>>()
    // );

    assert_eq!(true, pairs[0].0.set.contains(&2u32));
    assert_eq!(true, pairs[0].0.set.contains(&4u32));
    assert_eq!(true, pairs[0].1.set.contains(&6u32));
    assert_eq!(true, pairs[0].1.set.contains(&8u32));
    assert_eq!(true, pairs[5].0.set.contains(&2u32));
    assert_eq!(true, pairs[5].0.set.contains(&6u32));
    assert_eq!(true, pairs[5].1.set.contains(&4u32));
    assert_eq!(true, pairs[5].1.set.contains(&8u32));
}

#[test]
fn test_either_is_subset() {
    let pairs = get_pairs(&TESTPAIRS.to_string());

    let mut cnt = 0;
    for pair in pairs {
        println!("Pair: {:?}", pair);
        println!("Either is subset: {}", pair.0.either_is_subset(&pair.1));

        if pair.0.either_is_subset(&pair.1) {
            cnt += 1
        }
    }

    assert_eq!(cnt, 2);
}

#[test]
fn test_overlap() {
    let pairs = get_pairs(&TESTPAIRS.to_string());

    let mut olap_cnt = 0;
    for pair in pairs {
        if pair.0.overlaps_with(&pair.1) {
            println!("Yes: {:?} overlaps with {:?}\n----", pair.0, pair.1);
            olap_cnt += 1
        }
    }
    assert_eq!(4, olap_cnt);
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("src/puzzle.txt")?;
    let pairs = get_pairs(&contents);

    let mut cnt = 0;
    for pair in &pairs {
        if pair.0.either_is_subset(&pair.1) {
            cnt += 1
        }
    }
    println!("Fully contained sets: {}", cnt);

    let olaps: Vec<bool> = pairs
        .iter()
        .map(|pair| pair.0.overlaps_with(&pair.1))
        .collect();

    println!(
        "Pairs with any overlap: {:?}",
        olaps.iter().map(|b| *b as u32).sum::<u32>()
    );

    Ok(())
}
