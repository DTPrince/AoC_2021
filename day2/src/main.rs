/*
 * Derek Prince
 * AoC 2021 Day 2
 * bust out the regex again
 */
use std::fs::File;
use std::io::{Error, BufReader, BufRead};
use regex::Regex;

#[derive(Clone, Default, Debug)]
struct Instruction {
    // I seem to recall casting strings to chars created some uggo ownership issues
    direction: char,
    num: usize,
}

fn read_input() -> Result<Vec<String>, Error> {
    let path = "./input/input.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let lines = buffered.lines().map(|l| l.unwrap()).collect();
    Ok(lines)
}

fn clean_input(fcontents: Vec<String>) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    let re = Regex::new(r"([a-z]){1}(?:\w)+(?:\s){1}(\d){1}").unwrap();
    // loop through file contents using the regex to extract relevant info and populate instructions vec
    for line in fcontents {
        let rcaptures = re.captures(&*line).unwrap();
        let mut instruction = Instruction::default();
        instruction.direction = rcaptures.get(1)
            .map_or('\0', |s| s.as_str().trim().parse::<char>().unwrap());
        instruction.num = rcaptures.get(2)
            .map_or(0, |s| s.as_str().parse::<usize>().unwrap());
        instructions.push(instruction);
    }
    instructions
}

fn navigate_pt1(instructions: &Vec<Instruction>) {
    let mut depth = 0;
    let mut hpos = 0;
    for instruction in instructions {
        match instruction.direction {
            'f' => hpos = hpos + instruction.num,
            'u' => depth = depth - instruction.num,
            'd' => depth = depth + instruction.num,
            _ => println!("Nop found"),
        }
    }
    println!("Position is: {},{}, x*y={}", depth, hpos, depth*hpos);
}

fn navigate_pt2(instructions: &Vec<Instruction>) {
    let mut depth = 0;
    let mut hpos = 0;
    let mut aim = 0;
    for instruction in instructions {
        match instruction.direction {
            'f' => {
                hpos = hpos + instruction.num;
                depth = depth + aim*instruction.num;
            },
            'u' => aim = aim - instruction.num,
            'd' => aim = aim + instruction.num,
            _ => println!("Nop found"),
        }
    }
    println!("Position is: {},{}, x*y={}", depth, hpos, depth*hpos);
}

fn main() -> Result<(), Error> {
    let fcontents = read_input()?;
    let instructions = clean_input(fcontents);

    navigate_pt1(&instructions);
    navigate_pt2(&instructions);
    
    Ok(())
}
