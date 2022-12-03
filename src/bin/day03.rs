#![feature(iter_array_chunks)]
///
/// Created `get_common_item`,
/// a highly-generic function that uses associated types and const generics.
///
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

fn main() {
    const PUZZLE_DATA: &str = include_str!("day03/puzzle.txt");
    println!("Part1: {}", part1(PUZZLE_DATA));
    println!("Part2: {}", part2(PUZZLE_DATA));
}

fn part1(data: &str) -> u32 {
    let priorities = get_priority_mapping();

    data.lines()
        .map(|s| {
            let len = s.len();
            if len % 2 != 0 {
                panic!("Expected {s} to have an even length");
            };
            let mid = len / 2;

            let iterators = [s[..mid].chars(), s[mid..].chars()];
            get_common_item(iterators).into_iter().next().unwrap()
        })
        .map(|c| priorities.get(&c).unwrap())
        .sum()
}

fn part2(data: &str) -> u32 {
    let priorities = get_priority_mapping();

    data.lines()
        .array_chunks()
        .map(|[x, y, z]| {
            *get_common_item([x.chars(), y.chars(), z.chars()])
                .iter()
                .next()
                .unwrap()
        })
        .map(|c| priorities.get(&c).unwrap())
        .sum()
}

fn get_common_item<T, I, const N: usize>(iterators: [T; N]) -> Vec<I>
where
    T: Iterator<Item = I>,
    I: std::fmt::Debug + Eq + Hash + Clone + Copy,
{
    let mut hash_sets = iterators
        .into_iter()
        .map(|iter| HashSet::<_>::from_iter(iter));

    let first_hs = hash_sets.next().unwrap();

    hash_sets
        .fold(first_hs, |acc, hs| acc.intersection(&hs).cloned().collect())
        .into_iter()
        .collect()
}

fn get_priority_mapping() -> HashMap<char, u32> {
    ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx as u32))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = include_str!("day03/sample.txt");

    // #[rstest]
    // #[case(vec!["abc", "bcd"], vec!["b", "c"])]
    // fn test_get_common_items(#[case] input: Vec<String>, #[case] expected: Vec<String>) {
    //     let input = input.into_iter().map(|s| s.chars()).collect();
    //     assert_eq!(get_common_item(input), expected)
    // }

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 157);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), 70);
    }
}
