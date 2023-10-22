use std::fs;
use regex::Regex;
use itertools::Itertools;

fn part_1(input: &String) -> i32 {
    let max_cals: i32 = Regex::new(r"\n\n")
        .unwrap()
        .split(&input)
        .map(|elf| 
             elf.split_whitespace()
                .map(|food| food.parse::<i32>().unwrap())
                .sum()
        ).max()
        .unwrap();

    return max_cals;
}

fn part_2(input: &String) -> i32 {
    let sum_of_max_3_cals: i32 = Regex::new(r"\n\n")
        .unwrap()
        .split(&input)
        .map(|elf| 
             elf.split_whitespace()
                .map(|food| food.parse::<i32>().unwrap())
                .sum::<i32>()
        )
        .sorted()
        .rev()
        .take(3)
        .sum();

    return sum_of_max_3_cals;
}

fn main() {
    let input_string = fs::read_to_string("input.txt").unwrap();
    println!("part 1: {:}", part_1(&input_string));
    println!("part 2: {:}", part_2(&input_string));
}
