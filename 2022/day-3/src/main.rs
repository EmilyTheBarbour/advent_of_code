use std::fs;
use std::collections::HashSet;

use itertools::Itertools;

fn part_1(input: &String) -> i32 {
    let total = input.split_whitespace()
        .map(|sack|{
            // use this for caching the compute of the unique chars
            let second_half = &sack[sack.len() / 2..]
                .chars()
                .collect::<HashSet<_>>();

            return i32::from(
                *(&sack[..sack.len() / 2]
                    .chars()
                    .collect::<HashSet<_>>()
                    .iter()
                    .filter(|f| 
                            second_half.contains(&f)
                    )
                    .collect::<String>()
                    .as_bytes()
                    .iter()
                    .map(|c| if c.is_ascii_uppercase() { c - (b'A' - 1) + 26 } else { c - (b'a' - 1) })
                    .nth(0)
                    .unwrap()
                ));
        }
    ).sum::<i32>();

    return total;
}

fn part_2(input: &String) -> i32 {
    let total = input.split_whitespace()
        .into_iter()
        .chunks(3)
        .into_iter()
        .map(|group|{
            return i32::from(group.into_iter()
                .map(|bag| bag.chars().collect::<HashSet<_>>())
                .reduce(|a, b| {
                    a.into_iter().filter(|c| b.contains(c)).collect()
                })
                .unwrap()
                .iter()
                .map(|c| if c.is_ascii_uppercase() { *c as u8 - (b'A' - 1) + 26} else { *c as u8 - (b'a' - 1)})
                .nth(0)
                .unwrap()
            );
        }
    ).sum::<i32>();

    return total;
}

fn main() {
    let input_string = fs::read_to_string("input.txt").unwrap();
    println!("part 1: {:}", part_1(&input_string));
    println!("part 2: {:}", part_2(&input_string));
}
