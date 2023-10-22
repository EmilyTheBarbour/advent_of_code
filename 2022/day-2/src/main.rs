use std::fs;
use std::collections::HashMap;

fn part_1(input: &String) -> i32 {
    let strategy = HashMap::from([
        ("A X", 1 + 3),
        ("A Y", 2 + 6),
        ("A Z", 3 + 0),

        ("B X", 1 + 0),
        ("B Y", 2 + 3),
        ("B Z", 3 + 6),

        ("C X", 1 + 6),
        ("C Y", 2 + 0),
        ("C Z", 3 + 3),
    ]);

    let score: i32 = input.split("\n")
        .map(|f| strategy[f])
        .sum();

    return score;
}

fn part_2(input: &String) -> i32 {
    let strategy = HashMap::from([
        ("A X", 3 + 0),
        ("A Y", 1 + 3),
        ("A Z", 2 + 6),

        ("B X", 1 + 0),
        ("B Y", 2 + 3),
        ("B Z", 3 + 6),

        ("C X", 2 + 0),
        ("C Y", 3 + 3),
        ("C Z", 1 + 6),
    ]);

    let score: i32 = input.split("\n")
        .map(|f| strategy[f])
        .sum();

    return score;
}

fn main() {
    let input_string = fs::read_to_string("input.txt").unwrap();
    println!("part 1: {:}", part_1(&input_string));
    println!("part 2: {:}", part_2(&input_string));
}
