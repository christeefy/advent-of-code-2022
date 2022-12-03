#![feature(iter_array_chunks)]
///
/// Created `get_common_item`,
/// a highly-generic function that uses associated types and const generics.
///

use std::collections::HashSet;
use std::hash::Hash;

fn main() {
    const PUZZLE_DATA: &str = include_str!("day03/puzzle.txt");
    println!("Part1: {}", part1(PUZZLE_DATA.to_owned()));
    println!("Part2: {}", part2(PUZZLE_DATA.to_owned()));
}

fn part1(data: String) -> u32 {
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
        .map(get_priority)
        .sum()
}

fn part2(data: String) -> u32 {
    data.lines()
        .array_chunks()
        .map(|[x, y, z]| {
            *get_common_item([x.chars(), y.chars(), z.chars()])
                .iter()
                .next()
                .unwrap()
        })
        .map(get_priority)
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

fn get_priority(c: char) -> u32 {
    const UPPERCASE_A_ORDINAL: u32 = 'A' as u32;
    const LOWERCASE_A_ORDINAL: u32 = 'a' as u32;

    match c.is_uppercase() {
        true => c as u32 - UPPERCASE_A_ORDINAL + 27,
        false => c as u32 - LOWERCASE_A_ORDINAL + 1,
    }
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
        assert_eq!(part1(DATA.to_owned()), 157);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA.to_owned()), 70);
    }
}
