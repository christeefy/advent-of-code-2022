use std::cmp::Ordering;
use std::str::FromStr;

use advent_of_code_2022::rock_paper_scissors::RockPaperScissors;

fn main() {
    const PUZZLE_DATA: &str = include_str!("day02/puzzle.txt");

    println!(
        "Part1: {}\nPart2: {}",
        compute_scores(PUZZLE_DATA),
        compute_scores_clarified(PUZZLE_DATA)
    );
}

pub fn compute_scores(data: &str) -> u32 {
    data.split('\n')
        .map(|s| {
            let (opponent, me) = s.split_once(' ').unwrap();
            (
                RockPaperScissors::from_str(opponent).unwrap(),
                RockPaperScissors::from_str(me).unwrap(),
            )
        })
        .map(|(opponent_move, my_move)| my_move.score(&opponent_move))
        .sum()
}

pub fn compute_scores_clarified(data: &str) -> u32 {
    data.split('\n')
        .map(|s| {
            let (opponent, result) = s.split_once(' ').unwrap();
            (
                RockPaperScissors::from_str(opponent).unwrap(),
                match result {
                    "X" => Ordering::Less,
                    "Y" => Ordering::Equal,
                    "Z" => Ordering::Greater,
                    s => panic!("Unable to convert {s}"),
                },
            )
        })
        .map(|(other, ordering)| {
            RockPaperScissors::get_move_to_be(ordering, other.clone()).score(&other)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = include_str!("day02/sample.txt");

    #[test]
    fn test_compute_scores() {
        assert_eq!(compute_scores(DATA), 15)
    }

    #[test]
    fn test_compute_scores_clarified() {
        assert_eq!(compute_scores_clarified(DATA), 12)
    }
}
