/*
 * Derek Prince
 * AoC 2021 Day 3
 * Finding the most common bit value per bit
 */

use std::fmt::Display;
use std::fs::{File, read};
use std::io::{Error, BufReader, BufRead};

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
fn find_most_common(list: &Vec<String>, position: usize) -> u8 {
    let mut sum = 0;
    for number in list.clone() {
        if number.as_bytes()[position] == '1' as u8 {
            sum = sum + 1;
        }
    }
    if sum  >= (list.len() / 2) {
        return 1;
    }
    0
}

fn find_o2(fcontent: &Vec<String>) -> i32 {
    // just gonna clone and remove all wrong results for easy looping.
    let mut pile_o_shit = fcontent.clone();
    //could do an early break, we'll see how that looks
    let width = fcontent.iter().peekable().len(); // 4 in example input
    let mut position_key: usize = 0;

        for position in 0..width {
            let common = find_most_common(&pile_o_shit, position_key);

            // have to remove indexes after loop because length is loop count
            let mut trimmings : Vec<usize> = Vec::new();

            for line in pile_o_shit {
                if line.as_bytes()[position_key] != common {
                    pile_o_shit.pop();
                }
            }

        position_key = position_key + 1;
        }
    println!("Pile o shit length: {}, value: {}", pile_o_shit.len(), pile_o_shit.iter().next().unwrap());
    0
}

fn find_co2(fcontent: &Vec<String>) -> i32 {

    0
}

fn main() -> Result<(), Error> {
    let fcontents = read_input()?;
    let (gamma, epsilon) = find_gamma_epsilon(&fcontents);
    println!("Gamma: {:b} Epsilon: {:b}, Power: {}", gamma, epsilon, gamma*epsilon);

    let o2 = find_o2(&fcontents);
    let co2 = find_co2(&fcontents);

    Ok(())
}
