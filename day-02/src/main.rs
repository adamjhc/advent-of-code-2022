fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}

fn part_one() -> usize {
    utils::read_string("./input.txt")
        .lines()
        .map(|round| {
            let mut turns = round.split_ascii_whitespace();
            let turn_opponent = turns.next().unwrap();
            let turn_mine = turns.next().unwrap();

            let score_shape = match turn_mine {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => panic!(),
            };

            let score_outcome = match (turn_opponent, turn_mine) {
                ("A", "X") => 3,
                ("B", "X") => 0,
                ("C", "X") => 6,
                ("A", "Y") => 6,
                ("B", "Y") => 3,
                ("C", "Y") => 0,
                ("A", "Z") => 0,
                ("B", "Z") => 6,
                ("C", "Z") => 3,
                _ => panic!(),
            };

            score_shape + score_outcome
        })
        .sum()
}

fn part_two() -> usize {
    utils::read_string("./input.txt")
        .lines()
        .map(|round| {
            let mut turns = round.split_ascii_whitespace();
            let turn_opponent = turns.next().unwrap();
            let turn_mine = turns.next().unwrap();

            let score_outcome = match turn_mine {
                "X" => 0,
                "Y" => 3,
                "Z" => 6,
                _ => panic!(),
            };

            let score_shape = match (turn_opponent, turn_mine) {
                ("A", "X") => 3,
                ("B", "X") => 1,
                ("C", "X") => 2,
                ("A", "Y") => 1,
                ("B", "Y") => 2,
                ("C", "Y") => 3,
                ("A", "Z") => 2,
                ("B", "Z") => 3,
                ("C", "Z") => 1,
                _ => panic!(),
            };

            score_outcome + score_shape
        })
        .sum()
}
