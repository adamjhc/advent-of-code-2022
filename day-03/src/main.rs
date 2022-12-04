use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}

fn part_one() -> u32 {
    include_str!("../input.txt")
        .lines()
        .map(|line| {
            let rucksack_one = &line[..line.len() / 2];
            let rucksack_two = &line[line.len() / 2..line.len()];

            let set_one: HashSet<char> = HashSet::from_iter(rucksack_one.chars());
            let set_two: HashSet<char> = HashSet::from_iter(rucksack_two.chars());

            let item_type = set_one.intersection(&set_two).next().unwrap();

            get_priority(item_type)
        })
        .sum()
}

fn part_two() -> u32 {
    include_str!("../input.txt")
        .lines()
        .tuples()
        .map(|group: (&str, &str, &str)| {
            let mut set_one: HashSet<char> = HashSet::from_iter(group.0.chars());

            set_one.retain(|e| group.1.contains(*e));
            set_one.retain(|e| group.2.contains(*e));

            get_priority(set_one.iter().next().unwrap())
        })
        .sum()
}

fn get_priority(item_type: &char) -> u32 {
    *item_type as u32
        - if item_type.is_ascii_lowercase() {
            96
        } else {
            38
        }
}
