use std::collections::HashSet;

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}

fn part_one() -> usize {
    let chars: Vec<char> = include_str!("../input.txt").chars().collect();
    chars
        .windows(4)
        .position(|window| HashSet::<&char>::from_iter(window).len() == 4)
        .unwrap()
        + 4
}

fn part_two() -> usize {
    let chars: Vec<char> = include_str!("../input.txt").chars().collect();
    chars
        .windows(14)
        .position(|window| HashSet::<&char>::from_iter(window).len() == 14)
        .unwrap()
        + 14
}
