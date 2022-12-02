fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}

fn part_one() -> usize {
    get_elves_calorie_sums().into_iter().max().unwrap()
}

fn part_two() -> usize {
    let mut calories = get_elves_calorie_sums();
    calories.sort_unstable();
    let len = calories.len();
    calories[len - 1] + calories[len - 2] + calories[len - 3]
}

fn get_elves_calorie_sums() -> Vec<usize> {
    utils::read_string("./input.txt")
        .split("\n\n")
        .map(|elf_calories| {
            elf_calories
                .split_ascii_whitespace()
                .map(|calorie| calorie.parse::<usize>().unwrap())
                .sum()
        })
        .collect()
}
