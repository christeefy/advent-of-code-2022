use crate::utils::transpose;
use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, one_of},
    character::complete::{char, newline, space0, space1},
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

fn crate_id(s: &str) -> IResult<&str, char> {
    delimited(char('['), one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ"), char(']'))(s)
}

fn empty_crate(s: &str) -> IResult<&str, char> {
    delimited(char(' '), char(' '), char(' '))(s)
}

fn crate_line(s: &str) -> IResult<&str, Vec<char>> {
    separated_list1(char(' '), alt((crate_id, empty_crate)))(s)
}

fn stack_ids(s: &str) -> IResult<&str, Vec<u32>> {
    delimited(space1, separated_list1(space1, complete::u32), space0)(s)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Command {
    pub num_crates_to_move: usize,
    pub src_stack: u32,
    pub dst_stack: u32,
}

fn command(s: &str) -> IResult<&str, Command> {
    let (s, num_crates_to_move) = delimited(tag("move "), complete::u32, tag(" from "))(s)?;
    let (s, (src_stack, dst_stack)) = separated_pair(complete::u32, tag(" to "), complete::u32)(s)?;
    Ok((
        s,
        Command {
            num_crates_to_move: num_crates_to_move.try_into().unwrap(),
            src_stack,
            dst_stack,
        },
    ))
}

pub fn parse(s: &str) -> (HashMap<u32, Vec<char>>, Vec<Command>) {
    let (s, crates_untransposed) =
        terminated(separated_list1(newline, crate_line), newline)(s).unwrap();
    let (s, stack_ids) = terminated(stack_ids, many1(newline))(s).unwrap();
    let (_, commands) = separated_list1(newline, command)(s).unwrap();

    let crates_array = transpose(crates_untransposed)
        .into_iter()
        .map(|crates| {
            let mut crates = crates
                .into_iter()
                .filter(|&x| x != ' ')
                .collect::<Vec<char>>();
            crates.reverse();
            crates
        })
        .collect::<Vec<Vec<char>>>();

    let hm = HashMap::from_iter(stack_ids.into_iter().zip(crates_array));

    (hm, commands)
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("[A]", "A")]
    #[case("[E]", "E")]
    fn crate_id_works(#[case] input: &str, #[case] expected: char) {
        let (remaining, output) = crate_id(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(output, expected);
    }

    #[rstest]
    #[case("[A] [B] [C]", vec!['A', 'B', 'C'])]
    #[case("    [E]", vec![' ', 'E'])]
    #[case("        [E]", vec![' ', ' ', 'E'])]
    #[case("    [E]    ", vec![' ', 'E', ' '])]
    fn crate_line_works(#[case] input: &str, #[case] expected: Vec<char>) {
        let (remaining, output) = crate_line(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(output, expected);
    }

    #[rstest]
    #[case(" 1 ", vec![1])]
    #[case(" 1   2 ", vec![1, 2])]
    #[case(" 1   2   5 ", vec![1, 2, 5])]
    #[case(" 1   2   5", vec![1, 2, 5])]
    fn stack_ids_works(#[case] input: &str, #[case] expected: Vec<u32>) {
        let (remaining, output) = stack_ids(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(output, expected);
    }

    #[rstest]
    #[case("move 1 from 2 to 1", Command { num_crates_to_move: 1, src_stack: 2, dst_stack: 1})]
    #[case("move 3 from 1 to 3", Command { num_crates_to_move: 3, src_stack: 1, dst_stack: 3})]
    #[case("move 2 from 2 to 1", Command { num_crates_to_move: 2, src_stack: 2, dst_stack: 1})]
    fn command_works(#[case] input: &str, #[case] expected: Command) {
        let (remaining, output) = command(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(output, expected);
    }

    #[test]
    fn parse_works() {
        let data = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3";

        let (stacks, commands) = parse(data);

        let mut expected_stacks = HashMap::new();
        expected_stacks.insert(1, vec!['Z', 'N']);
        expected_stacks.insert(2, vec!['M', 'C', 'D']);
        expected_stacks.insert(3, vec!['P']);

        // let expected_stacks = vec![
        //     vec!["Z", "N"],
        //     vec!["M", "C", "D"],
        //     vec!["P"]
        // ];

        let expected_commands = vec![
            Command {
                num_crates_to_move: 1,
                src_stack: 2,
                dst_stack: 1,
            },
            Command {
                num_crates_to_move: 3,
                src_stack: 1,
                dst_stack: 3,
            },
        ];

        assert_eq!(stacks, expected_stacks);
        assert_eq!(commands, expected_commands);
    }
}
