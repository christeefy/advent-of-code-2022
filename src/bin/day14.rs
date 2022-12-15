#![feature(iter_collect_into, array_windows)]
use std::collections::BTreeMap;

use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::{sequence::separated_pair, IResult};

fn main() {
    const PUZZLE_DATA: &str = include_str!("day14/puzzle.txt");
    println!("Part1: {}", part1(PUZZLE_DATA));
    println!("Part2: {}", part2(PUZZLE_DATA));
}

fn part1(data: &str) -> usize {
    let (_, mut scan) = cave_scan(data).unwrap();
    scan.drip_sand();
    scan.sand_count
}

fn part2(data: &str) -> usize {
    let (_, mut scan) = cave_scan_with_floor(data).unwrap();
    scan.drip_sand();
    scan.sand_count
}

type Position = (usize, usize);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Item {
    Rock,
    Sand,
    SandSource,
}

impl Item {
    fn to_symbol(&self) -> char {
        match self {
            Self::Rock => '#',
            Self::Sand => 'o',
            Self::SandSource => '+',
        }
    }
}

const SAND_SOURCE_POSITION: Position = (500, 0);

struct CaveScan {
    data: BTreeMap<Position, Item>,
    sand_count: usize,
}

impl CaveScan {
    fn new(data: BTreeMap<Position, Item>) -> Self {
        Self {
            data,
            sand_count: 0,
        }
    }

    fn drip_sand(&mut self) {
        let mut current_sand = SAND_SOURCE_POSITION;
        let lowest_rock_depth = *self.data.iter().map(|((_, y), _)| y).max().unwrap();
        loop {
            // println!("{self}\n");
            if current_sand.1 >= lowest_rock_depth || self.data.contains_key(&SAND_SOURCE_POSITION)
            {
                return;
            }

            let lower_left = (current_sand.0 - 1, current_sand.1 + 1);
            let down = (current_sand.0, current_sand.1 + 1);
            let lower_right = (current_sand.0 + 1, current_sand.1 + 1);

            match (
                self.data.get(&lower_left),
                self.data.get(&down),
                self.data.get(&lower_right),
            ) {
                (Some(_), Some(_), Some(_)) => {
                    self.sand_count += 1;
                    self.data.insert(current_sand, Item::Sand);
                    current_sand = SAND_SOURCE_POSITION;
                }
                (_, None, _) => {
                    current_sand = down;
                }
                (None, _, _) => {
                    current_sand = lower_left;
                }
                (_, _, None) => {
                    current_sand = lower_right;
                }
            }
        }
    }
}

impl std::fmt::Display for CaveScan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x_min = self.data.iter().map(|((x, _), _)| x).min().unwrap();
        let x_max = self.data.iter().map(|((x, _), _)| x).max().unwrap();
        let y_max = self.data.iter().map(|((_, y), _)| y).max().unwrap();

        let mut arr = vec![vec!['.'; x_max - x_min + 1]; *y_max + 1];

        self.data
            .iter()
            .for_each(|(&(x, y), item)| arr[y][x - x_min] = item.to_symbol());

        // Add sand source
        arr[SAND_SOURCE_POSITION.1][SAND_SOURCE_POSITION.0 - x_min] = Item::SandSource.to_symbol();

        write!(
            f,
            "{}",
            arr.iter()
                .enumerate()
                .map(|(i, inner)| format!("{} {}", i, inner.iter().collect::<String>()))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

fn rock_position(input: &str) -> IResult<&str, Position> {
    map(
        separated_pair(complete::u32, complete::char(','), complete::u32),
        |(x, y)| (x as usize, y as usize),
    )(input)
}

fn rock_line(input: &str) -> IResult<&str, Vec<Position>> {
    map(separated_list1(tag(" -> "), rock_position), |positions| {
        let mut out = vec![];
        positions.array_windows().for_each(|&[(x1, y1), (x2, y2)]| {
            out.push((x1, y1));

            match (x1, y1, x2, y2) {
                (x1, y1, x2, y2) if x1 == x2 && y1 < y2 => (0..(y2 - y1))
                    .map(|y_delta| (x1, y1 + y_delta))
                    .collect_into(&mut out),
                (x1, y1, x2, y2) if x1 == x2 && y1 > y2 => (0..(y1 - y2))
                    .map(|y_delta| (x1, y1 - y_delta))
                    .collect_into(&mut out),
                (x1, y1, x2, y2) if y1 == y2 && x1 < x2 => (0..(x2 - x1))
                    .map(|x_delta| (x1 + x_delta, y1))
                    .collect_into(&mut out),
                (x1, y1, x2, y2) if y1 == y2 && x1 > x2 => (0..(x1 - x2))
                    .map(|x_delta| (x1 - x_delta, y1))
                    .collect_into(&mut out),
                _ => panic!("Invalid point combination obtained: ({x1}, {y1}) -> ({x2}, {y2})"),
            };

            out.push((x2, y2));
        });

        out
    })(input)
}

fn cave_scan(input: &str) -> IResult<&str, CaveScan> {
    map(
        separated_list1(complete::newline, rock_line),
        |rock_lines| {
            CaveScan::new(
                rock_lines
                    .into_iter()
                    .flatten()
                    .map(|pos| (pos, Item::Rock))
                    .collect(),
            )
        },
    )(input)
}

fn cave_scan_with_floor(input: &str) -> IResult<&str, CaveScan> {
    let (output, mut cave_scan) = cave_scan(input)?;

    let x_min = *cave_scan.data.iter().map(|((x, _), _)| x).min().unwrap();
    let x_max = *cave_scan.data.iter().map(|((x, _), _)| x).max().unwrap();
    let y_max = *cave_scan.data.iter().map(|((_, y), _)| y).max().unwrap();

    const PADDING: usize = 400;

    cave_scan.data.extend(
        ((x_min - PADDING)..=(x_max + PADDING))
            .cartesian_product([y_max + 2])
            .map(|pos| (pos, Item::Rock)),
    );

    Ok((output, cave_scan))
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = include_str!("day14/sample.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 24);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), 93);
    }

    #[test]
    fn test_part2_puzzle() {
        const PUZZLE_DATA: &str = include_str!("day14/puzzle.txt");
        assert_eq!(part2(PUZZLE_DATA), 28594);
    }
}
