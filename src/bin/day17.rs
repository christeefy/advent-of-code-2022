#![feature(iter_intersperse)]
use std::cmp::max;
use std::collections::HashSet;

use std::io::Write;

use nom::character::complete;
use nom::error::Error;
use nom::{branch::alt, combinator::map, multi::many1};

fn main() {
    const PUZZLE_DATA: &str = include_str!("day17/puzzle.txt");
    // println!("Part1: {}", part1(PUZZLE_DATA));
    println!("Part2: {}", part2(PUZZLE_DATA));
}

fn part1(data: &str) -> u64 {
    let movement_pattern = parse(data);
    process(movement_pattern, 2022)
}

fn part2(data: &str) -> u64 {
    let movement_pattern = parse(data);
    process(movement_pattern, 1_000_000)
    // process(movement_pattern, 1_000_000_000_000)
}

fn process(movement_pattern: Vec<Movement>, n_iter: usize) -> u64 {
    dbg!(&movement_pattern.len());
    let mut movements = movement_pattern
        .iter()
        .intersperse(&Movement::Down)
        .chain(&[Movement::Down])
        .cycle();
    let mut board = Board::new();

    let mut file = std::fs::File::create("data.csv").unwrap();

    for i in 0..n_iter {
        let mut block = make_block(i, &board);
        loop {
            // let mut board_clone = board.clone();
            // let positions = block.positions();
            // board_clone.add_block(positions);
            // println!("Run: {i}\n\n{board_clone}\n");

            let movement = movements.next().unwrap();
            match (movement, &block.try_move(*movement, &board)) {
                (Movement::Down, None) => break,
                _ => (),
            }
        }
        let prev_max_height = board.max_height;
        board.add_block(block.positions());
        let max_height_delta = board.max_height - prev_max_height;
        writeln!(file, "{i},{}", max_height_delta);
    }

    board.max_height
}

type Position = (i64, i64);

fn shift(pos: Position, x: i64, y: i64) -> Position {
    (pos.0 + x, pos.1 + y)
}

#[derive(Debug, Clone, Copy)]
enum Movement {
    Left,
    Right,
    Down,
}

#[derive(Debug, Clone)]
struct Board {
    data: HashSet<Position>,
    max_height: u64,
    max_width: u8,
}

impl Board {
    fn new() -> Self {
        const MAX_WIDTH: u8 = 7;
        Self {
            data: HashSet::new(),
            max_height: 0,
            max_width: MAX_WIDTH,
        }
    }

    fn contains(&self, pos: &Position) -> Option<bool> {
        match pos {
            &(x, _) if x < 0 || x >= self.max_width as i64 => None,
            &(_, y) if y < 0 => None,
            pos => Some(self.data.contains(pos)),
        }
    }

    fn add_block(&mut self, positions: Vec<Position>) {
        let mut block_max_height: u64 = positions
            .iter()
            .map(|&(_, y)| y)
            // .inspect(|y| println!("{y}"))
            .max()
            .unwrap()
            .try_into()
            .unwrap();
        block_max_height += 1;

        self.max_height = max(self.max_height, block_max_height);
        self.data.extend(positions);
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut arr = vec![vec!['.'; self.max_width as usize]; self.max_height as usize];

        self.data
            .iter()
            .for_each(|&(x, y)| arr[y as usize][x as usize] = '#');

        // Reverse the rows
        arr.reverse();

        write!(
            f,
            "{}",
            arr.iter()
                .enumerate()
                .map(|(i, row)| format!(
                    "{} {}",
                    arr.len() - (i + 1),
                    row.iter().collect::<String>()
                ))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

trait Block: TryMove {
    fn positions(&self) -> Vec<Position>;
}

trait TryMove {
    fn move_left(&mut self, board: &Board) -> Option<()>;
    fn move_right(&mut self, board: &Board) -> Option<()>;
    fn move_down(&mut self, board: &Board) -> Option<()>;
    fn try_move(&mut self, movement: Movement, board: &Board) -> Option<()> {
        match movement {
            Movement::Left => self.move_left(board),
            Movement::Right => self.move_right(board),
            Movement::Down => self.move_down(board),
        }
    }
}

#[derive(Debug)]
struct Bar {
    // Origin located at: *...
    origin: Position,
}

impl Bar {
    fn new(board: &Board) -> Self {
        Self {
            origin: (2, (board.max_height + 3).try_into().unwrap()),
        }
    }
}

impl TryMove for Bar {
    fn move_left(&mut self, board: &Board) -> Option<()> {
        // x*...
        match board.contains(&shift(self.origin, -1, 0)) {
            None => None,
            Some(true) => None,
            Some(false) => {
                self.origin = shift(self.origin, -1, 0);
                Some(())
            }
        }
    }

    fn move_right(&mut self, board: &Board) -> Option<()> {
        // *...x
        match board.contains(&shift(self.origin, 4, 0)) {
            None => None,
            Some(true) => None,
            Some(false) => {
                self.origin = shift(self.origin, 1, 0);
                Some(())
            }
        }
    }

    fn move_down(&mut self, board: &Board) -> Option<()> {
        // *...
        // xxxx
        match (
            board.contains(&shift(self.origin, 0, -1)),
            board.contains(&shift(self.origin, 1, -1)),
            board.contains(&shift(self.origin, 2, -1)),
            board.contains(&shift(self.origin, 3, -1)),
        ) {
            (Some(false), Some(false), Some(false), Some(false)) => {
                self.origin = shift(self.origin, 0, -1);
                Some(())
            }
            _ => None,
        }
    }
}

impl Block for Bar {
    fn positions(&self) -> Vec<Position> {
        vec![
            self.origin,
            shift(self.origin, 1, 0),
            shift(self.origin, 2, 0),
            shift(self.origin, 3, 0),
        ]
    }
}

#[derive(Debug)]
struct Cross {
    // Origin located at:
    //  .
    // .*.
    //  .
    origin: Position,
}

impl Cross {
    fn new(board: &Board) -> Self {
        Self {
            origin: (3, (board.max_height + 4).try_into().unwrap()),
        }
    }
}

impl TryMove for Cross {
    fn move_left(&mut self, board: &Board) -> Option<()> {
        //  x.
        // x.*.
        //  x.
        match (
            board.contains(&shift(self.origin, -1, 1)),
            board.contains(&shift(self.origin, -2, 0)),
            board.contains(&shift(self.origin, -1, -1)),
        ) {
            (Some(false), Some(false), Some(false)) => {
                self.origin = shift(self.origin, -1, 0);
                Some(())
            }
            _ => None,
        }
    }

    fn move_right(&mut self, board: &Board) -> Option<()> {
        //  .x
        // .*.x
        //  .x
        match (
            board.contains(&shift(self.origin, 1, 1)),
            board.contains(&shift(self.origin, 2, 0)),
            board.contains(&shift(self.origin, 1, -1)),
        ) {
            (Some(false), Some(false), Some(false)) => {
                self.origin = shift(self.origin, 1, 0);
                Some(())
            }
            _ => None,
        }
    }

    fn move_down(&mut self, board: &Board) -> Option<()> {
        //  .
        // .*.
        // x.x
        //  x
        match (
            board.contains(&shift(self.origin, -1, -1)),
            board.contains(&shift(self.origin, 0, -2)),
            board.contains(&shift(self.origin, 1, -1)),
        ) {
            (Some(false), Some(false), Some(false)) => {
                self.origin = shift(self.origin, 0, -1);
                Some(())
            }
            _ => None,
        }
    }
}

impl Block for Cross {
    fn positions(&self) -> Vec<Position> {
        vec![
            self.origin,
            shift(self.origin, -1, 0),
            shift(self.origin, 1, 0),
            shift(self.origin, 0, -1),
            shift(self.origin, 0, 1),
        ]
    }
}

#[derive(Debug)]
struct Corner {
    // Origin located at:
    //   .
    //   .
    // *..
    origin: Position,
}

impl Corner {
    fn new(board: &Board) -> Self {
        Self {
            origin: (2, (board.max_height + 3).try_into().unwrap()),
        }
    }
}

impl TryMove for Corner {
    fn move_left(&mut self, board: &Board) -> Option<()> {
        //   x.
        //   x.
        // x*..
        match (
            board.contains(&shift(self.origin, -1, 0)),
            board.contains(&shift(self.origin, 1, 1)),
            board.contains(&shift(self.origin, 1, 2)),
        ) {
            (Some(false), Some(false), Some(false)) => {
                self.origin = shift(self.origin, -1, 0);
                Some(())
            }
            _ => None,
        }
    }

    fn move_right(&mut self, board: &Board) -> Option<()> {
        //   .x
        //   .x
        // *..x
        match (
            board.contains(&shift(self.origin, 3, 0)),
            board.contains(&shift(self.origin, 3, 1)),
            board.contains(&shift(self.origin, 3, 2)),
        ) {
            (Some(false), Some(false), Some(false)) => {
                self.origin = shift(self.origin, 1, 0);
                Some(())
            }
            _ => None,
        }
    }

    fn move_down(&mut self, board: &Board) -> Option<()> {
        //   .
        //   .
        // *..
        // xxx
        match (
            board.contains(&shift(self.origin, 0, -1)),
            board.contains(&shift(self.origin, 1, -1)),
            board.contains(&shift(self.origin, 2, -1)),
        ) {
            (Some(false), Some(false), Some(false)) => {
                self.origin = shift(self.origin, 0, -1);
                Some(())
            }
            _ => None,
        }
    }
}

impl Block for Corner {
    fn positions(&self) -> Vec<Position> {
        vec![
            self.origin,
            shift(self.origin, 1, 0),
            shift(self.origin, 2, 0),
            shift(self.origin, 2, 1),
            shift(self.origin, 2, 2),
        ]
    }
}

#[derive(Debug)]
struct Pole {
    // Origin located at:
    // .
    // .
    // .
    // *
    origin: Position,
}

impl Pole {
    fn new(board: &Board) -> Self {
        Self {
            origin: (2, (board.max_height + 3).try_into().unwrap()),
        }
    }
}

impl TryMove for Pole {
    fn move_left(&mut self, board: &Board) -> Option<()> {
        // x.
        // x.
        // x.
        // x*
        match (
            board.contains(&shift(self.origin, -1, 0)),
            board.contains(&shift(self.origin, -1, 1)),
            board.contains(&shift(self.origin, -1, 2)),
            board.contains(&shift(self.origin, -1, 3)),
        ) {
            (Some(false), Some(false), Some(false), Some(false)) => {
                self.origin = shift(self.origin, -1, 0);
                Some(())
            }
            _ => None,
        }
    }

    fn move_right(&mut self, board: &Board) -> Option<()> {
        // .x
        // .x
        // .x
        // *x
        match (
            board.contains(&shift(self.origin, 1, 0)),
            board.contains(&shift(self.origin, 1, 1)),
            board.contains(&shift(self.origin, 1, 2)),
            board.contains(&shift(self.origin, 1, 3)),
        ) {
            (Some(false), Some(false), Some(false), Some(false)) => {
                self.origin = shift(self.origin, 1, 0);
                Some(())
            }
            _ => None,
        }
    }

    fn move_down(&mut self, board: &Board) -> Option<()> {
        // .
        // .
        // .
        // *
        // x
        match board.contains(&shift(self.origin, 0, -1)) {
            Some(false) => {
                self.origin = shift(self.origin, 0, -1);
                Some(())
            }
            _ => None,
        }
    }
}

impl Block for Pole {
    fn positions(&self) -> Vec<Position> {
        vec![
            self.origin,
            shift(self.origin, 0, 1),
            shift(self.origin, 0, 2),
            shift(self.origin, 0, 3),
        ]
    }
}

#[derive(Debug)]
struct Square {
    // Origin located at:
    // ..
    // *.
    origin: Position,
}

impl Square {
    fn new(board: &Board) -> Self {
        Self {
            origin: (2, (board.max_height + 3).try_into().unwrap()),
        }
    }
}

impl TryMove for Square {
    fn move_left(&mut self, board: &Board) -> Option<()> {
        // x..
        // x*.
        match (
            board.contains(&shift(self.origin, -1, 0)),
            board.contains(&shift(self.origin, -1, 1)),
        ) {
            (Some(false), Some(false)) => {
                self.origin = shift(self.origin, -1, 0);
                Some(())
            }
            _ => None,
        }
    }

    fn move_right(&mut self, board: &Board) -> Option<()> {
        // ..x
        // *.x
        match (
            board.contains(&shift(self.origin, 2, 0)),
            board.contains(&shift(self.origin, 2, 1)),
        ) {
            (Some(false), Some(false)) => {
                self.origin = shift(self.origin, 1, 0);
                Some(())
            }
            _ => None,
        }
    }

    fn move_down(&mut self, board: &Board) -> Option<()> {
        // ..
        // *.
        // xx
        match (
            board.contains(&shift(self.origin, 0, -1)),
            board.contains(&shift(self.origin, 1, -1)),
        ) {
            (Some(false), Some(false)) => {
                self.origin = shift(self.origin, 0, -1);
                Some(())
            }
            _ => None,
        }
    }
}

impl Block for Square {
    fn positions(&self) -> Vec<Position> {
        vec![
            self.origin,
            shift(self.origin, 1, 0),
            shift(self.origin, 0, 1),
            shift(self.origin, 1, 1),
        ]
    }
}

fn make_block(i: usize, board: &Board) -> Box<dyn Block> {
    match i % 5 {
        0 => Box::new(Bar::new(board)),
        1 => Box::new(Cross::new(board)),
        2 => Box::new(Corner::new(board)),
        3 => Box::new(Pole::new(board)),
        4 => Box::new(Square::new(board)),
        _ => unreachable!(),
    }
}

fn parse(input: &str) -> Vec<Movement> {
    let (_, movements) = many1(map(
        alt((
            complete::char::<&str, Error<&str>>('<'),
            complete::char('>'),
        )),
        |x| match x {
            '<' => Movement::Left,
            '>' => Movement::Right,
            c => panic!("Invalid character \'{c}\'"),
        },
    ))(input)
    .unwrap();
    movements
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = include_str!("day17/sample.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 3068);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), 1514285714288);
    }
}
