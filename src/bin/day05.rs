#![feature(map_many_mut, iter_collect_into)]
use advent_of_code_2022::io::parse_day5::{parse, Command};
use std::collections::HashMap;

fn main() {
    const PUZZLE_DATA: &str = include_str!("day05/puzzle.txt");
    println!("Part1: {}", part1(PUZZLE_DATA));
    println!("Part2: {}", part2(PUZZLE_DATA));
}

fn part1(data: &str) -> String {
    let (mut stacks, commands) = parse(data);
    reorder_stacks(&mut stacks, commands, true);
    get_top_crates(&stacks)
}

fn part2(data: &str) -> String {
    let (mut stacks, commands) = parse(data);
    reorder_stacks(&mut stacks, commands, false);
    get_top_crates(&stacks)
}

fn reorder_stacks(stacks: &mut HashMap<u32, Vec<char>>, commands: Vec<Command>, reverse: bool) {
    for command in commands {
        let [src, dst] = stacks
            .get_many_mut([&command.src_stack, &command.dst_stack])
            .unwrap();
        let final_length = src.len() - command.num_crates_to_move;

        if reverse {
            src.drain(final_length..).rev().collect_into(dst);
        } else {
            src.drain(final_length..).collect_into(dst);
        }
    }
}

fn get_top_crates(stacks: &HashMap<u32, Vec<char>>) -> String {
    (1..=stacks.len())
        .map(|idx| *stacks.get(&(idx as u32)).unwrap().last().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = include_str!("day05/sample.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), "CMZ");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), "MCD");
    }
}
