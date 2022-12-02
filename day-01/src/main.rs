fn main() {
    println!("{}", part_one());
}

fn part_one() -> usize {
    utils::read_string("./input.txt")
        .split("\n\n")
        .map(|elf_calories| {
            elf_calories
                .split_ascii_whitespace()
                .map(|calorie| calorie.parse::<usize>().unwrap())
                .sum()
        })
        .max()
        .unwrap()
}
