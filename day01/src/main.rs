use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = "./input.txt";
    let input = File::open(path).expect("couldn't open file");
    let buffered = BufReader::new(input);

    let mut calories = Vec::new();

    let mut current_calories = 0;

    for line in buffered.lines() {
        if let Ok(ip) = line {
            match ip {
                ip if ip.is_empty() => {
                    calories.push(current_calories);
                    current_calories = 0
                }
                _ => {
                    let num: u64 = ip.trim().parse().expect("Illegal input");
                    current_calories += num
                }
            }
        }
    }
    calories.sort();

    let max_value = *calories.iter().max().expect("Couldn't find max value");

    println!("Solution for day 01/1: {max_value}");

    let max_three_values_sum: u64 = calories.iter().rev().take(3).sum();

    println!("Solution for day 01/2: {max_three_values_sum}");
}
