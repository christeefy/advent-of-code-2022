use std::collections::HashSet;

use nom::{
    character::complete::{self, newline, one_of},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn main() {
    const PUZZLE_DATA: &str = include_str!("day09/puzzle.txt");
    println!("Part1: {}", part1(PUZZLE_DATA));
    println!("Part2: {}", part2(PUZZLE_DATA));
}

fn part1(data: &str) -> usize {
    let (_, moves) = parse(data).unwrap();
    let mut rope = Rope::new(2);

    for (move_action, n_times) in moves {
        for _ in 0..n_times {
            rope.move_rope(move_action);
        }
    }
    rope.tail_history.len()
}

fn part2(data: &str) -> usize {
    let (_, moves) = parse(data).unwrap();
    let mut rope = Rope::new(10);

    for (move_action, n_times) in moves {
        for _ in 0..n_times {
            rope.move_rope(move_action);
        }
    }
    rope.tail_history.len()
}

type Position = (i32, i32);

#[derive(Debug, Clone, Copy)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn new(direction: char) -> Self {
        match direction {
            'U' => Self::Up,
            'D' => Self::Down,
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Invalid direction obtained"),
        }
    }
}

#[derive(Debug, Default)]
struct Rope {
    knots: Vec<Position>,
    tail_history: HashSet<Position>,
}

impl Rope {
    pub fn new(num_knots: usize) -> Self {
        Self {
            knots: vec![Default::default(); num_knots],
            tail_history: Default::default(),
        }
    }

    pub fn move_rope(&mut self, move_action: Move) {
        self.move_head(move_action);
        self.move_body();
        self.update_tail_history();
    }

    fn move_head(&mut self, move_action: Move) {
        let pos = self.knots[0];
        self.knots[0] = match move_action {
            Move::Up => (pos.0, pos.1 + 1),
            Move::Down => (pos.0, pos.1 - 1),
            Move::Left => (pos.0 - 1, pos.1),
            Move::Right => (pos.0 + 1, pos.1),
        };
    }

    fn move_body(&mut self) {
        for i in 1..self.knots.len() {
            let prev = self.knots[i - 1];
            let current = self.knots[i];

            let delta_x = prev.0 - current.0;
            let delta_y = prev.1 - current.1;

            self.knots[i] = match (delta_x, delta_y) {
                // Handle centre 3x3 square
                (-1 | 0 | 1, -1 | 0 | 1) => current,
                // Handle four corners
                (2, 2) => (current.0 + 1, current.1 + 1),
                (-2, 2) => (current.0 - 1, current.1 + 1),
                (2, -2) => (current.0 + 1, current.1 - 1),
                (-2, -2) => (current.0 - 1, current.1 - 1),
                // Handle edges
                (2, y) => (current.0 + 1, current.1 + y),
                (-2, y) => (current.0 - 1, current.1 + y),
                (x, 2) => (current.0 + x, current.1 + 1),
                (x, -2) => (current.0 + x, current.1 - 1),
                // Catch-all
                invalid => panic!("Invalid move ({invalid:?})"),
            };
        }
    }

    fn update_tail_history(&mut self) {
        self.tail_history.insert(*self.knots.last().unwrap());
    }
}

fn parse(s: &str) -> IResult<&str, Vec<(Move, i32)>> {
    separated_list1(
        newline,
        map(
            separated_pair(one_of("UDLR"), complete::char(' '), complete::i32),
            |(direction, distance)| (Move::new(direction), distance),
        ),
    )(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = include_str!("day09/sample.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), 1);
    }

    #[test]
    fn test_part2_complex() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(part2(input), 36)
    }
}
