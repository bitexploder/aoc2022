use regex::Regex;
use std::error::Error;
use std::fs;

const STACKS: &str = &"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

#[derive(Debug)]
struct Instr {
    count: usize,
    from_stack: usize,
    to_stack: usize,
}

impl Instr {
    fn parse(s: &str) -> Self {
        let re = Regex::new(r"[a-z ]+([0-9]+)[a-z ]+([0-9]+)[a-z ]+([0-9]+)").unwrap();
        let caps = re.captures(s).unwrap();

        Self {
            count: caps.get(1).map_or(0, |m| m.as_str().parse().unwrap()),
            from_stack: caps.get(2).map_or(0, |m| m.as_str().parse().unwrap()) - 1,
            to_stack: caps.get(3).map_or(0, |m| m.as_str().parse().unwrap()) - 1,
        }
    }

    // keep_order = false for p1, keep_order = true for p2
    fn execute(&self, stacks: &mut Vec<String>, keep_order: bool) {
        let fstack = &stacks[self.from_stack];
        let range = fstack.len() - self.count..;
        let drained: &Vec<char> = &stacks[self.from_stack].drain(range).collect::<Vec<char>>();

        if !keep_order {
            for d in drained.iter().rev() {
                let _ = &stacks[self.to_stack].push(*d);
            }
        } else {
            for d in drained.iter() {
                let _ = &stacks[self.to_stack].push(*d);
            }
        }
    }
}

// Transpose string rows into columns to make it easier to parse the stacks
fn transpose(v: Vec<&str>) -> Vec<String> {
    let mut new_rows: Vec<String> = Vec::new();

    for (_, row) in v.iter().enumerate() {
        for (j, col) in row.chars().enumerate() {
            if new_rows.len() <= j {
                new_rows.push("".to_string())
            }

            new_rows[j].push(col);
        }
    }
    new_rows
}

// Clean the rows of non-essential characters creating the stacks to be worked upon
fn clean_rows(v: Vec<String>) -> Vec<String> {
    let re = Regex::new(r"[\[\]0-9 ]").unwrap();

    let replaced = v
        .iter()
        .map(|s| re.replace_all(s, "").into_owned())
        .collect::<Vec<String>>();

    let mut cleaned: Vec<String> = Vec::new();

    for row in replaced {
        if row.len() > 0 {
            cleaned.push(row)
        }
    }

    cleaned
        .iter()
        .map(|s| s.chars().rev().collect::<String>())
        .collect()
}

// Load, clean, and prep the stacks and instructions
fn load_stack_and_instr(contents: &str) -> (Vec<String>, Vec<Instr>) {
    let stack_inst: Vec<&str> = contents.split("\n\n").collect();
    let rows = stack_inst[0].split("\n").collect::<Vec<&str>>();
    let res = transpose(rows);
    let cleaned = clean_rows(res);
    let inst_str = stack_inst[1].split("\n").collect::<Vec<&str>>();
    let instrs: Vec<Instr> = inst_str.iter().map(|s| Instr::parse(*s)).collect();

    return (cleaned, instrs);
}

#[test]
fn test_sample_input_part2() {
    let (mut stacks, instrs) = load_stack_and_instr(STACKS);

    for inst in instrs {
        // println!("Instr: {:?}", inst);
        println!("{:?}", stacks);
        inst.execute(&mut stacks, true);
    }

    let top: String = stacks
        .iter()
        .map(|stack| stack.get(stack.len() - 1..).unwrap())
        .collect::<_>();
    eprintln!("top = {:?}", top);

    assert_eq!(top, "MCD");
}

#[test]
fn test_sample_input_part1() {
    let (mut stacks, instrs) = load_stack_and_instr(STACKS);

    for inst in instrs {
        println!("{:?}", stacks);
        inst.execute(&mut stacks, false);
    }

    let top: String = stacks
        .iter()
        .map(|stack| stack.get(stack.len() - 1..).unwrap())
        .collect::<_>();
    eprintln!("top = {:?}", top);

    assert_eq!(top, "CMZ");
}

fn part1() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("src/puzzle.txt")?;

    let (mut stacks, instrs) = load_stack_and_instr(contents.as_str());

    for inst in instrs {
        inst.execute(&mut stacks, false);
    }

    let top_letters: String = stacks
        .iter()
        .map(|stack| stack.get(stack.len() - 1..).unwrap())
        .collect::<_>();
    eprintln!("Part 1 -- top_letters = {:?}", top_letters);

    Ok(())
}

fn part2() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("src/puzzle.txt")?;

    let (mut stacks, instrs) = load_stack_and_instr(contents.as_str());

    for inst in instrs {
        inst.execute(&mut stacks, true);
    }

    let top_letters: String = stacks
        .iter()
        .map(|stack| stack.get(stack.len() - 1..).unwrap())
        .collect::<_>();
    eprintln!("Part 2 -- top_letters = {:?}", top_letters);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    part1()?;
    part2()?;
    Ok(())
}
