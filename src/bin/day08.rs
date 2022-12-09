/// New crates used this time include:
/// - `ndarray`
/// - `rayon` (`par_bridge`9)
/// - `itertools` (`cartesian_product` and `fold_while`)
/// - `num-traits` (including the concept of generic `one`s)
/// Created a trait bound-heavy generic function `scenic_score`.
use itertools::{FoldWhile, Itertools};
use ndarray::{Array1, Array2};
use nom::{
    character::complete::{digit1, newline},
    combinator::map,
    error::Error,
    multi::separated_list1,
    IResult,
};
use num_traits::Unsigned;
use rayon::prelude::*;
use std::fmt::Debug;

/// Position of a tree
type Position = (usize, usize);

fn main() {
    const PUZZLE_DATA: &str = include_str!("day08/puzzle.txt");
    println!("Part1: {}", part1(PUZZLE_DATA));
    println!("Part2: {}", part2(PUZZLE_DATA));
}

fn part1(data: &str) -> usize {
    let array = parse(data);

    let inner_visible_trees = (1..(array.nrows() - 1))
        .cartesian_product(1..(array.ncols() - 1))
        .par_bridge()
        .filter(|&(row, col)| is_visible((row, col), &array))
        .count();

    let external_visible_trees = array.len() - (array.nrows() - 2) * (array.ncols() - 2);

    inner_visible_trees + external_visible_trees
}

fn part2(data: &str) -> usize {
    let array = parse(data);
    (1..(array.nrows() - 1))
        .cartesian_product(1..(array.ncols() - 1))
        .par_bridge()
        .map(|pos| scenic_score(pos, &array))
        .max()
        .unwrap()
}

/// Check if a tree is visible along its corresponding row or column
fn is_visible<T: Ord>(position: (usize, usize), arr: &Array2<T>) -> bool {
    let tree_height = arr.get(position).unwrap();

    get_trees(position, arr).iter().any(|tree_positions| {
        tree_positions
            .iter()
            .map(|&pos| arr.get(pos).unwrap())
            .max()
            .unwrap()
            < tree_height
    })
}

/// Check if a tree is visible along its corresponding row or column
fn scenic_score<T: Debug + Unsigned + PartialOrd + Copy>(
    position: (usize, usize),
    arr: &Array2<T>,
) -> usize {
    let tree_house_height = *arr.get(position).unwrap();

    get_trees(position, arr)
        .iter()
        .map(|trees| {
            // The get count of a vector of trees whose heights
            // are monotonically ascending
            trees
                .iter()
                .map(|&pos| *arr.get(pos).unwrap())
                .fold_while(vec![], |mut acc, height| {
                    acc.push(height);
                    if height >= tree_house_height {
                        FoldWhile::Done(acc)
                    } else {
                        FoldWhile::Continue(acc)
                    }
                })
                .into_inner()
                .len()
        })
        .product()
}

fn get_trees<T>(position: Position, arr: &Array2<T>) -> [Vec<Position>; 4] {
    let n_rows = arr.nrows();
    let n_cols = arr.ncols();

    let northern_trees = (0..position.0)
        .rev()
        .cartesian_product([position.1])
        .collect();
    let eastern_trees = [position.0]
        .into_iter()
        .cartesian_product((position.1 + 1)..n_cols)
        .collect();
    let southern_trees = ((position.0 + 1)..n_rows)
        .cartesian_product([position.1])
        .collect();
    let western_trees = [position.0]
        .into_iter()
        .cartesian_product((0..position.1).rev())
        .collect();

    [northern_trees, eastern_trees, southern_trees, western_trees]
}

fn row(s: &str) -> IResult<&str, Vec<u8>> {
    map(digit1, |x: &str| {
        x.chars()
            .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
            .collect()
    })(s)
}

fn parse(s: &str) -> Array2<u8> {
    let (_, output) = separated_list1::<_, _, _, Error<_>, _, _>(newline, row)(s).unwrap();
    let arr_height = output.len();
    let arr_width = output[0].len();
    let output = output.into_iter().flatten().collect::<Array1<_>>();
    output.into_shape((arr_height, arr_width)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    const DATA: &str = include_str!("day08/sample.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 21);
    }

    #[rstest]
    #[case((1, 2), 4)]
    #[case((3, 2), 8)]
    fn test_scenic_score(#[case] position: Position, #[case] expected: usize) {
        let arr = parse(DATA);
        assert_eq!(scenic_score(position, &arr), expected)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), 8);
    }
}
