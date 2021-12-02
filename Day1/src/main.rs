/*
 * AoC 2021
 * Day 1
 */
use std::fs::File;
use std::io::{Error, BufReader, BufRead};

fn read_input() -> Result<Vec<i32>, Error> {
    let path = "./input/input.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let lines = buffered.lines().map(|l| l.unwrap());
    let mut items : Vec<i32> = Vec::new();
    for line in lines {
        items.push(line.parse::<i32>().unwrap());   // we lose int parse errors here but whatever
    }
    Ok(items)
}

fn main() {
    let values = read_input().unwrap();

    // count sequential incerases
    let mut count = 0;
    let mut depth = values.iter().peekable();
    for _ in 0..values.len() {
        let current = depth.next().unwrap();
        let next = depth.peek();
        // just trying out the error handling. Should always peek invalid memory on end of file.
        match next {
            Some(next) => if (*next - current) > 0 {
                count = count + 1;
            },
            None => println!("Overshot your shot there bud"),
        }
    }

    // Part 2
    // Taking 3-measurement slices and comparing. Iterate+1, slice [n..n+3] vs [n+1..n+4] (or [n..=n+2])
    let mut window_sum = 0;
    for level in 0..values.len() - 3 {
        let current_window = &values[level..level+3];
        let next_window = &values[level+1..level+4];
        let current_window_sum : i32 = current_window.iter().sum();
        let next_window_sum : i32 = next_window.iter().sum();
        if current_window_sum < next_window_sum {
            window_sum = window_sum + 1;
        }
    }
    println!("Number of depth increases: {}", count);
    println!("Window Sum: {}", window_sum);

    // Though the number increased, it is still correct.
    // I suppose it makes sense since the numbers trend downward.
}
