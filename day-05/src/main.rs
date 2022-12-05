use regex::Regex;

fn main() {
    println!("{}", part_one());
}

fn part_one() -> String {
    let mut stacks = vec![
        vec!['D', 'B', 'J', 'V'],
        vec!['P', 'V', 'B', 'W', 'R', 'D', 'F'],
        vec!['R', 'G', 'F', 'L', 'D', 'C', 'W', 'Q'],
        vec!['W', 'J', 'P', 'M', 'L', 'N', 'D', 'B'],
        vec!['H', 'N', 'B', 'P', 'C', 'S', 'Q'],
        vec!['R', 'D', 'B', 'S', 'N', 'G'],
        vec!['Z', 'B', 'P', 'M', 'Q', 'F', 'S', 'H'],
        vec!['W', 'L', 'F'],
        vec!['S', 'V', 'F', 'M', 'R'],
    ];

    let regex = Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();
    include_str!("../input.txt")
        .lines()
        .skip(10)
        .map(|line| {
            let captures = regex.captures(line).unwrap();
            (
                captures[1].parse::<usize>().unwrap(),
                captures[2].parse::<usize>().unwrap() - 1,
                captures[3].parse::<usize>().unwrap() - 1,
            )
        })
        .for_each(|(quantity, from, to)| {
            for _ in 0..quantity {
                let moved = stacks[from].pop().unwrap();
                stacks[to].push(moved);
            }
        });

    stacks.iter().fold(String::new(), |mut output, stack| {
        output.push(*stack.last().unwrap());
        output
    })
}
