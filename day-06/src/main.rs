use std::collections::HashSet;

fn main() {
    println!("{}", part_one());
}

fn part_one() -> usize {
    let chars: Vec<char> = include_str!("../input.txt").chars().collect();
    chars
        .windows(4)
        .position(|window| HashSet::<&char>::from_iter(window).len() == 4)
        .unwrap()
        + 4
}
