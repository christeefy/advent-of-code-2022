use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};
use rayon::prelude::*;

fn main() {
    const PUZZLE_DATA: &str = include_str!("day15/puzzle.txt");
    println!("Part1: {}", part1(PUZZLE_DATA, 2_000_000));
    println!("Part2: {}", part2(PUZZLE_DATA, 4_000_000));
}

fn part1(data: &str, target_row: i64) -> usize {
    let (_, sensors_and_beacons) = parse(data).unwrap();

    let mut beaconless_ranges = sensors_and_beacons
        .par_iter()
        .filter_map(|&((s, _), (b, _))| within_manhattan(s, b, target_row))
        .collect::<Vec<_>>();

    beaconless_ranges.sort_by(|a, b| a[0].cmp(&b[0]));

    beaconless_ranges
        .into_iter()
        .fold(vec![], |mut acc: Vec<[i64; 2]>, new| match acc.pop() {
            Some(last) => {
                acc.push(match (last[0], last[1], new[0], new[1]) {
                    (a, b, c, d) if a <= c && b >= d => [a, b],
                    (a, b, c, d) if a <= c && b < d => [a, d],
                    (a, b, c, d) if a > c && b >= d => [c, b],
                    (a, b, c, d) if a > c && b < d => [c, d],
                    (a, b, c, d) => panic!("Got invalid range {a}..={b} {c}..={d}"),
                });
                acc
            }
            None => vec![new],
        })
        .iter()
        .map(|[a, b]| (b - a) as usize + 1)
        .sum::<usize>()
        - 1
}

fn part2(data: &str, u_bound: i64) -> i64 {
    let (_, sensors_and_beacons) = parse(data).unwrap();

    let radar = Radar {
        data: sensors_and_beacons
            .clone()
            .into_iter()
            .flat_map(|(sensor, beacon)| [sensor, beacon].into_iter())
            .collect(),
    };

    (0..=u_bound)
        .par_bridge()
        .find_map_any(|target_row| {
            let mut beaconless_ranges = sensors_and_beacons
                .iter()
                .enumerate()
                .filter_map(|(i, &((s, _), (b, _)))| {
                    within_manhattan(s, b, target_row).map(|x| (i, x))
                })
                .map(|(i, [a, b])| (i, [a.clamp(0, u_bound), b.clamp(0, u_bound)]))
                .collect::<Vec<_>>();

            beaconless_ranges.sort_by(|(_, a), (_, b)| a[0].cmp(&b[0]));

            let collapsed_ranges = beaconless_ranges.into_iter().map(|(_, pos)| pos).fold(
                vec![],
                |mut acc: Vec<[i64; 2]>, new| match acc.pop() {
                    Some(last) => {
                        match (last[0], last[1], new[0], new[1]) {
                            (a, b, c, d) if (b + 1) < c => {
                                acc.push([a, b]);
                                acc.push([c, d]);
                            }
                            (a, b, c, d) if a <= c && b >= d => acc.push([a, b]),
                            (a, b, c, d) if a <= c && b < d => acc.push([a, d]),
                            (a, b, c, d) if a > c && b >= d => acc.push([c, b]),
                            (a, b, c, d) if a > c && b < d => acc.push([c, d]),
                            (a, b, c, d) => panic!("Got invalid range {a}..={b} {c}..={d}"),
                        };
                        acc
                    }
                    None => vec![new],
                },
            );

            if collapsed_ranges.len() == 2 {
                let x = collapsed_ranges[0][1] + 1;
                let y = target_row;
                Some(x * 4_000_000 + y)
            } else {
                None
            }
        })
        .unwrap()
}

type Position = (i64, i64);

fn within_manhattan(s: Position, b: Position, target_row: i64) -> Option<[i64; 2]> {
    let manhattan_dist = (s.0 - b.0).abs() + (s.1 - b.1).abs();

    let row_abs_diff = (s.1 - target_row).abs();
    if row_abs_diff <= manhattan_dist {
        let l_bound = s.0 + -manhattan_dist + row_abs_diff;
        let u_bound = s.0 + manhattan_dist - row_abs_diff;
        Some([l_bound, u_bound])
    } else {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Item {
    Sensor,
    Beacon,
    Beaconless,
}

impl Item {
    fn to_char(&self) -> char {
        match self {
            Self::Sensor => 'S',
            Self::Beacon => 'B',
            Self::Beaconless => '#',
        }
    }
}

struct Radar {
    data: HashMap<Position, Item>,
}

impl std::fmt::Display for Radar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x_min = self.data.iter().map(|((x, _), _)| x).min().unwrap();
        let x_max = self.data.iter().map(|((x, _), _)| x).max().unwrap();
        let y_min = self.data.iter().map(|((_, y), _)| y).min().unwrap();
        let y_max = self.data.iter().map(|((_, y), _)| y).max().unwrap();

        let mut arr = vec![vec!['.'; (x_max - x_min + 1) as usize]; (y_max - y_min + 1) as usize];

        self.data.iter().for_each(|(&(x, y), item)| {
            arr[(y - y_min) as usize][(x - x_min) as usize] = item.to_char()
        });

        write!(
            f,
            "{}",
            arr.iter()
                .enumerate()
                .map(|(i, inner)| format!(
                    "{:>5} {}",
                    i as i64 + y_min,
                    inner.iter().collect::<String>()
                ))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

fn coordinate(input: &str) -> IResult<&str, Position> {
    preceded(
        tag("x="),
        separated_pair(complete::i64, tag(", y="), complete::i64),
    )(input)
}

fn sensor_and_beacon(input: &str) -> IResult<&str, ((Position, Item), (Position, Item))> {
    map(
        preceded(
            tag("Sensor at "),
            separated_pair(coordinate, tag(": closest beacon is at "), coordinate),
        ),
        |(sensor_pos, beacon_pos)| ((sensor_pos, Item::Sensor), (beacon_pos, Item::Beacon)),
    )(input)
}

fn parse(input: &str) -> IResult<&str, Vec<((Position, Item), (Position, Item))>> {
    separated_list1(newline, sensor_and_beacon)(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = include_str!("day15/sample.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA, 10), 26);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA, 20), 56_000_011);
    }
}
