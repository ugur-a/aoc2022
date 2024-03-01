use itertools::Itertools;

pub fn p2(file: &str) -> u32 {
    file.lines()
        // parse a round as pair of moves
        .map(|round| round.split_whitespace().collect_tuple().unwrap())
        .map(|(opp_move, your_move)| {
            // score based on what you played
            (match (opp_move, your_move) {
                ("A", "Y") | ("B", "X") | ("C", "Z") => 1,
                ("A", "Z") | ("B", "Y") | ("C", "X") => 2,
                ("A", "X") | ("B", "Z") | ("C", "Y") => 3,
                _ => unreachable!(),
            })
            // score based on round outcome
            + match your_move {
                "X" => 0,
                "Y" => 3,
                "Z" => 6,
                _ => unreachable!(),
            }
        })
        .sum()
}

pub fn p1(file: &str) -> u32 {
    file.lines()
        .map(|round| round.split_whitespace().collect_tuple().unwrap())
        .map(|(opp_move, your_move)| {
            // score based on round outcome
            (match (opp_move, your_move) {
            ("A", "Z") | ("B", "X") | ("C", "Y") => 0,
            ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
            ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
            _ => unreachable!(),
        })
        // score based on what you played
        + match your_move {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => unreachable!(),
        }
        })
        .sum()
}
