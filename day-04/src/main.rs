fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}

fn part_one() -> usize {
    include_str!("../input.txt")
        .lines()
        .filter(|line| {
            let mut split = line.split(',');
            let range_one = parse_range(split.next().unwrap());
            let range_two = parse_range(split.next().unwrap());

            (range_one.0 <= range_two.0 && range_two.1 <= range_one.1)
                || (range_two.0 <= range_one.0 && range_one.1 <= range_two.1)
        })
        .count()
}

fn part_two() -> usize {
    include_str!("../input.txt")
        .lines()
        .filter(|line| {
            let mut split = line.split(',');
            let range_one = parse_range(split.next().unwrap());
            let range_two = parse_range(split.next().unwrap());

            (range_one.0..=range_one.1)
                .any(|section| (range_two.0..=range_two.1).contains(&section))
        })
        .count()
}

fn parse_range(range: &str) -> (usize, usize) {
    let mut split = range.split('-');
    (
        split.next().unwrap().parse::<usize>().unwrap(),
        split.next().unwrap().parse::<usize>().unwrap(),
    )
}
