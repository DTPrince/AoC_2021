/*
 * Derek Prince
 * AoC 2021 Day 3
 * Finding the most common bit value per bit
 */

use std::fmt::Display;
use std::fs::{File, read};
use std::io::{Error, BufReader, BufRead};
use std::{collections::BTreeSet};   // A smart man once told me "You're using the wrong tool for the job" aka vectors for everything

fn read_input() -> Result<Vec<String>, Error> {
    let path = "./input/input.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let lines = buffered.lines().map(|l| l.unwrap()).collect();
    Ok(lines)
}

// just going to sum bits and compare the file length I suppose. Same thing as counting
fn find_gamma_epsilon(fcontents: &Vec<String>) -> (i32, i32){
    let bit_width: usize = fcontents.iter().peekable()
        .peek().unwrap().len();
    let length = fcontents.len();
    let mut sums: Vec<i32> = vec![0; bit_width];
    for line in fcontents {
        // lets just iterate over the bits, w/e. Masking is slightly hairy with dynamic length
        let mut iter = line.chars();
        let count = line.chars().count();
        for c in 0..count {
            match iter.next().unwrap() {
                '0' => (),
                '1' => sums[c] = sums[c] + 1,
                _ => println!("No match found for {} in {}", c, line),
            }
        }
    }

    let mut gamma = String::new();
    let mut epsilon = String::new();
    // I was just going to xor gama with hxF but this works just fine I suppose
    for i in 0..sums.len() {
        if sums[i] > (length / 2) as i32 {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }
    let bgamma = i32::from_str_radix(gamma.as_str(), 2).unwrap();
    let bepsilon = i32::from_str_radix(epsilon.as_str(), 2).unwrap();
    (bgamma, bepsilon)
}

// inverse of most common will be the same behavior wanted for co2 scrubber
// checks for number of 1s. Returns true for half or more, false for less than half.
// aka true  = 1 majority
//     false = 0 majority
fn find_most_common(list: &Vec<String>, position: usize) -> char {
    // dividing rounds down so it was HIGHLY favoring 1 results. Just going to count both like a peanut brain
    let mut sum_1 = 0;
    let mut sum_0 = 0;
    for number in list {
        if number.chars().nth(position).unwrap() == '1' {
            sum_1 = sum_1 + 1;
        } else {
            sum_0 = sum_0 + 1;
        }
    }
    if sum_1  >= sum_0 {
        return '1';
    }
    '0'
}

fn find_o2(fcontent: &Vec<String>) -> String {
    // just gonna clone and remove all wrong results for easy looping.
    let mut pile_o_shit = fcontent.clone();
    //could do an early break, we'll see how that looksugly
    let width = pile_o_shit.first().unwrap().len();
    let mut position_key: usize = 0;

        for _ in 0..width {
            let most_common = find_most_common(&pile_o_shit, position_key);

            let mut illegals: BTreeSet<String> = BTreeSet::new();
            for line in &pile_o_shit {
                // println!("Line {}, key: {}, common: {}, line[key]: {}", line, position_key, most_common, line.chars().nth(position_key).unwrap());
                if line.chars().nth(position_key).unwrap() != most_common {
                    illegals.insert(line.clone());
                }
            }
            if pile_o_shit.len() > 1 {
                pile_o_shit.retain(|x| !illegals.contains(x));
            }
            position_key = position_key + 1;
        }
    println!("Pile o shit length: {}, value: {}", pile_o_shit.len(), pile_o_shit.first().unwrap());
    pile_o_shit.iter().next().unwrap().clone()

}

fn find_co2(fcontent: &Vec<String>) -> String {
    // just gonna clone and remove all wrong results for easy looping.
    let mut pile_o_shit = fcontent.clone();
    //could do an early break, we'll see how that looksugly
    let width = pile_o_shit.first().unwrap().len();
    let mut position_key: usize = 0;

    for _ in 0..width {
        let most_common = find_most_common(&pile_o_shit, position_key);

        let mut illegals: BTreeSet<String> = BTreeSet::new();
        for line in &pile_o_shit {
            // println!("Line {}, key: {}, common: {}, line[key]: {}", line, position_key, most_common, line.chars().nth(position_key).unwrap());
            if line.chars().nth(position_key).unwrap() == most_common {
                illegals.insert(line.clone());
            }
        }
        if pile_o_shit.len() > 1 {
            pile_o_shit.retain(|x| !illegals.contains(x));
        }
        position_key = position_key + 1;
    }
    println!("Pile o shit length: {}, value: {}", pile_o_shit.len(), pile_o_shit.first().unwrap());
    pile_o_shit.iter().next().unwrap().clone()

}

fn main() -> Result<(), Error> {
    let fcontents = read_input()?;
    let (gamma, epsilon) = find_gamma_epsilon(&fcontents);
    println!("Gamma: {:b} Epsilon: {:b}, Power: {}", gamma, epsilon, gamma*epsilon);

    let o2 = find_o2(&fcontents);
    let co2 = find_co2(&fcontents);

    let num_o2 = i32::from_str_radix(o2.as_str(), 2).unwrap();
    let num_co2 = i32::from_str_radix(co2.as_str(), 2).unwrap();
    println!("o2 rating: {}, co2 rating: {}, life support rating: {}", o2, co2, num_o2*num_co2);

    Ok(())
}
