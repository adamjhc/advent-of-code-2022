use std::collections::HashSet;

fn main() {
    println!("{}", part_one())
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

            item_type.to_owned() as u32
                - if item_type.is_ascii_lowercase() {
                    96
                } else {
                    38
                }
        })
        .sum()
}
