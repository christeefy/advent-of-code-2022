use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{self, alpha1, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, terminated},
    IResult,
};

fn main() {
    const PUZZLE_DATA: &str = include_str!("day07/puzzle.txt");
    println!("Part1: {}", part1(PUZZLE_DATA));
    println!("Part2: {}", part2(PUZZLE_DATA));
}

fn part1(data: &str) -> u32 {
    let (_, operations) = parse(data).unwrap();

    let (_, sizes) = operations
        .into_iter()
        .fold((vec![], HashMap::new()), calculate_sizes);

    sizes
        .iter()
        .filter_map(|(_, &v)| if v < 100000 { Some(v) } else { None })
        .sum()
}

fn part2(data: &str) -> u32 {
    let (_, operations) = parse(data).unwrap();
    let (_, sizes) = operations
        .into_iter()
        .fold((vec![], HashMap::new()), calculate_sizes);

    const DEVICE_SIZE: u32 = 70_000_000;
    const UPDATE_SIZE: u32 = 30_000_000;
    let update_space_deficit = UPDATE_SIZE - (DEVICE_SIZE - sizes.get(&vec!["/"]).unwrap());

    sizes
        .iter()
        .filter_map(|(_, &v)| {
            if v >= update_space_deficit {
                Some(v)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

fn calculate_sizes<'a>(
    (mut context, mut sizes): (Vec<&'a str>, HashMap<Vec<&'a str>, u32>),
    op: Operation<'a>,
) -> (Vec<&'a str>, HashMap<Vec<&'a str>, u32>) {
    match op {
        Operation::ChangeDirectory(ChangeDirectory::Root) => context.push("/"),
        Operation::ChangeDirectory(ChangeDirectory::Up) => {
            context.pop();
        }
        Operation::ChangeDirectory(ChangeDirectory::Down(name)) => context.push(name),
        Operation::ListDirectory(filesystem_objs) => {
            let current_directory_size: u32 = filesystem_objs.iter().sum();

            // Update sizes for directory hierarchy
            for i in 0..context.len() {
                sizes
                    .entry(context[0..=i].to_vec())
                    .and_modify(|size| {
                        *size += current_directory_size;
                    })
                    .or_insert(current_directory_size);
            }
        }
    }

    (context, sizes)
}

#[derive(Debug)]
enum ChangeDirectory<'a> {
    Root,
    Up,
    Down(&'a str),
}

#[derive(Debug)]
enum Operation<'a> {
    ChangeDirectory(ChangeDirectory<'a>),
    ListDirectory(Vec<u32>),
}

fn file_size(s: &str) -> IResult<&str, Option<u32>> {
    map(
        terminated(complete::u32, is_a(" qwertyuiopasdfghjklzxcvbnm.")),
        |x| Some(x),
    )(s)
}

fn directory_size(s: &str) -> IResult<&str, Option<u32>> {
    map(preceded(tag("dir "), alpha1), |_| None)(s)
}

fn ls_command(s: &str) -> IResult<&str, Operation> {
    map(
        preceded(
            tag("$ ls\n"),
            separated_list1(newline, alt((file_size, directory_size))),
        ),
        |sizes| Operation::ListDirectory(sizes.iter().filter_map(|&x| x).collect()),
    )(s)
}

fn cd_command(s: &str) -> IResult<&str, Operation> {
    map(
        preceded(tag("$ cd "), alt((tag("/"), alpha1, tag("..")))),
        |input| {
            let input = match input {
                "/" => ChangeDirectory::Root,
                ".." => ChangeDirectory::Up,
                name => ChangeDirectory::Down(name),
            };
            Operation::ChangeDirectory(input)
        },
    )(s)
}

fn parse(s: &str) -> IResult<&str, Vec<Operation>> {
    separated_list1(newline, alt((cd_command, ls_command)))(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = include_str!("day07/sample.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 95437);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), 24933642);
    }
}
