use core::panic;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn get_compartments(input: &str) -> (&str, &str) {
    let split_at = input.len() / 2;
    let first = &input[..split_at];
    let last = &input[split_at..];
    (first, last)
}

fn get_duplicate(input: (&str, &str)) -> char {
    for c in input.0.chars() {
        if input.1.find(c).is_some() {
            return c;
        }
    }
    panic!("couldn't find duplicate")
}

fn get_priority(item: char) -> u32 {
    if item.is_uppercase() {
        item as u32 - 38
    } else {
        item as u32 - 96
    }
}

fn calculate_priority(input: &str) -> u32 {
    let compartments = get_compartments(input);
    let duplicate = get_duplicate(compartments);
    get_priority(duplicate)
}

fn calculate_priorities(input: Vec<String>) -> u32 {
    input.iter().map(|line| calculate_priority(line)).sum()
}

fn lines_from_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn unique_item(input: &Vec<String>) -> char {
    if input.len() != 3 {
        panic!("illegal group")
    }
    let candidates: Vec<char> = input[0]
        .chars()
        .filter(|c| input[1].chars().find(|d| d == c).is_some())
        .filter(|c| input[2].chars().find(|d| d == c).is_some())
        .collect();
    *candidates.first().expect("msg")
}

fn calculate_group_priorities(input: Vec<String>) -> u32 {
    let mut group: Vec<String> = Vec::new();
    let mut result = 0;
    for line in input {
        group.push(line);

        if group.len() == 3 {
            let unique_item = unique_item(&group);
            result += get_priority(unique_item);
            group = Vec::new();
        }
    }

    result
}

fn main() {
    let path = "./input.txt";
    let input = lines_from_file(path);
    let result = calculate_priorities(input);
    println!("Result for day 03/01: {result}");

    let input = lines_from_file(path);
    let result = calculate_group_priorities(input);
    println!("Result for day 03/02: {result}");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_compartments() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let expected = ("vJrwpWtwJgWr", "hcsFMMfFFhFp");

        assert_eq!(get_compartments(input), expected);
    }

    #[test]
    fn test_get_duplicate() {
        let input = ("vJrwpWtwJgWr", "hcsFMMfFFhFp");
        assert_eq!(get_duplicate(input), 'p');
    }

    #[test]
    fn test_get_priority_lowercase() {
        let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
        let mut counter = 1;
        alphabet.iter().for_each(|input| {
            assert_eq!(get_priority(*input), counter);
            counter += 1;
        })
    }

    #[test]
    fn test_get_priority_uppercase() {
        let alphabet: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
        let mut counter = 27;
        alphabet.iter().for_each(|input| {
            assert_eq!(get_priority(*input), counter);
            counter += 1;
        })
    }

    #[test]
    fn test_calculate_priority() {
        let puzzle_input: Vec<String> = [
            String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
            String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            String::from("PmmdzqPrVvPwwTWBwg"),
            String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
            String::from("ttgJtRGJQctTZtZT"),
            String::from("CrZsJsPPZsGzwwsLwLmpwMDw"),
        ]
        .to_vec();

        assert_eq!(calculate_priorities(puzzle_input), 157)
    }

    #[test]
    fn test_unique_item() {
        let puzzle_input: Vec<String> = [
            String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
            String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            String::from("PmmdzqPrVvPwwTWBwg"),
        ]
        .to_vec();

        assert_eq!(unique_item(&puzzle_input), 'r')
    }

    #[test]
    fn test_unique_item_2() {
        let puzzle_input: Vec<String> = [
            String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
            String::from("ttgJtRGJQctTZtZT"),
            String::from("CrZsJsPPZsGzwwsLwLmpwMDw"),
        ]
        .to_vec();

        assert_eq!(unique_item(&puzzle_input), 'Z')
    }

    #[test]
    fn test_calculate_group_priorities() {
        let puzzle_input: Vec<String> = [
            String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
            String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            String::from("PmmdzqPrVvPwwTWBwg"),
            String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
            String::from("ttgJtRGJQctTZtZT"),
            String::from("CrZsJsPPZsGzwwsLwLmpwMDw"),
        ]
        .to_vec();

        assert_eq!(calculate_group_priorities(puzzle_input), 70)
    }
}
