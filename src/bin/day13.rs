use std::cmp::Ordering;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, multispace1, newline},
    combinator::map,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};
use rayon::prelude::*;

fn main() {
    const PUZZLE_DATA: &str = include_str!("day13/puzzle.txt");
    println!("Part1: {}", part1(PUZZLE_DATA));
    println!("Part2: {}", part2(PUZZLE_DATA));
}

fn part1(data: &str) -> usize {
    let (_, packet_pairs) = packet_pairs(data).unwrap();

    packet_pairs
        .par_iter()
        .enumerate()
        .filter_map(|(i, (x, y))| if x < y { Some(i + 1) } else { None })
        .sum()
}
fn part2(data: &str) -> usize {
    let (_, packet_pairs) = packet_pairs(data).unwrap();
    let packet_2 = Packet::List(vec![Packet::List(vec![Packet::Scalar(2)])]);
    let packet_6 = Packet::List(vec![Packet::List(vec![Packet::Scalar(6)])]);

    let mut packets = packet_pairs
        .iter()
        .flat_map(|(left, right)| vec![left, right])
        .chain([&packet_2, &packet_6])
        .collect::<Vec<_>>();

    packets.sort();

    packets
        .into_iter()
        .enumerate()
        .filter_map(|(i, p)| {
            if p == &packet_2 || p == &packet_6 {
                Some(i + 1)
            } else {
                None
            }
        })
        .product()
}

// TODO: Implement Display
#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Scalar(u8),
    List(Vec<Packet>),
}

impl std::fmt::Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Scalar(x) => x.to_string(),
                Self::List(x) => {
                    format!("[{}]", x.iter().map(|elem| elem.to_owned()).join(","))
                }
            }
        )
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Scalar(x), Self::Scalar(y)) => x.cmp(y),
            (Self::List(x), Self::List(y)) => x.cmp(y),
            (Self::Scalar(x), Self::List(y)) => vec![Packet::Scalar(*x)].cmp(y),
            (Self::List(x), Self::Scalar(y)) => x.cmp(&vec![Packet::Scalar(*y)]),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn packet(input: &str) -> IResult<&str, Packet> {
    alt((
        delimited(
            complete::char('['),
            map(separated_list0(tag(","), packet), Packet::List),
            complete::char(']'),
        ),
        map(complete::u8, Packet::Scalar),
    ))(input)
}

fn packet_pairs(input: &str) -> IResult<&str, Vec<(Packet, Packet)>> {
    separated_list1(multispace1, separated_pair(packet, newline, packet))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = include_str!("day13/sample.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), 140);
    }
}
