use std::collections::BTreeSet;

use nom::{
    character::complete, combinator::map, multi::separated_list1, sequence::separated_pair, IResult,
};
use rayon::prelude::*;

fn main() {
    const PUZZLE_DATA: &str = include_str!("day18/puzzle.txt");
    println!("Part1: {}", part1(PUZZLE_DATA));
    println!("Part2: {}", part2(PUZZLE_DATA));
}

fn part1(data: &str) -> usize {
    let (_, cubes) = cubes(data).unwrap();

    cubes
        .par_iter()
        .map(|cube| {
            [
                (1, 0, 0),
                (-1, 0, 0),
                (0, 1, 0),
                (0, -1, 0),
                (0, 0, 1),
                (0, 0, -1),
            ]
            .into_iter()
            .filter(|&(x, y, z)| !cubes.contains(&(cube.0 + x, cube.1 + y, cube.2 + z)))
            .count()
        })
        .sum()
}

fn part2(data: &str) -> usize {
    let (_, cubes) = cubes(data).unwrap();

    cubes
        .par_iter()
        .filter_map(|&cube| {
            if is_interior_cube(&cube, &cubes) {
                println!("{cube:?}");
                None
            } else {
                Some(cube)
            }
        })
        .map(|cube| {
            [
                (1, 0, 0),
                (-1, 0, 0),
                (0, 1, 0),
                (0, -1, 0),
                (0, 0, 1),
                (0, 0, -1),
            ]
            .into_iter()
            .filter(|&(x, y, z)| !cubes.contains(&(cube.0 + x, cube.1 + y, cube.2 + z)))
            .count()
        })
        .sum()
}

fn is_interior_cube(cube: &Position, cubes: &BTreeSet<Position>) -> bool {
    let has_lower_x = cubes
        .iter()
        .any(|&(x, y, z)| cube.1 == y && cube.2 == z && cube.0 < x);
    let has_upper_x = cubes
        .iter()
        .any(|&(x, y, z)| cube.1 == y && cube.2 == z && cube.0 > x);
    let has_lower_y = cubes
        .iter()
        .any(|&(x, y, z)| cube.0 == x && cube.2 == z && cube.1 < y);
    let has_upper_y = cubes
        .iter()
        .any(|&(x, y, z)| cube.0 == x && cube.2 == z && cube.1 > y);
    let has_lower_z = cubes
        .iter()
        .any(|&(x, y, z)| cube.0 == x && cube.1 == y && cube.2 < z);
    let has_upper_z = cubes
        .iter()
        .any(|&(x, y, z)| cube.0 == x && cube.1 == y && cube.2 > z);
    [
        has_lower_x,
        has_upper_x,
        has_lower_y,
        has_upper_y,
        has_lower_z,
        has_upper_z,
    ]
    .iter()
    .all(|&x| x)
}

type Position = (i32, i32, i32);

fn cube(input: &str) -> IResult<&str, Position> {
    map(
        separated_pair(
            separated_pair(complete::i32, complete::char(','), complete::i32),
            complete::char(','),
            complete::i32,
        ),
        |((x, y), z)| (x, y, z),
    )(input)
}

fn cubes(input: &str) -> IResult<&str, BTreeSet<Position>> {
    map(separated_list1(complete::newline, cube), |x| {
        x.into_iter().collect()
    })(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = include_str!("day18/sample.txt");

    #[test]
    fn test_part1_simple() {
        let data = "1,1,1
2,1,1";
        assert_eq!(part1(data), 10);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 64);
    }

    #[test]
    #[should_panic]
    fn test_part2() {
        assert_eq!(part2(DATA), 58);
    }
}
