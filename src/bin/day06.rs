use std::collections::HashSet;

fn main() {
    const PUZZLE_DATA: &str = include_str!("day06/puzzle.txt");
    println!("Part1: {}", part1(PUZZLE_DATA));
    println!("Part2: {}", part2(PUZZLE_DATA));
}

fn part1(data: &str) -> usize {
    find_longest_unique_idx(data, 4)
}
fn part2(data: &str) -> usize {
    find_longest_unique_idx(data, 14)
}

fn find_longest_unique_idx(data: &str, seq_len: usize) -> usize {
    data
        .as_bytes()
        .windows(seq_len)
        .enumerate()
        .find_map(|(i, arr)| {
            if arr.iter().collect::<HashSet<&u8>>().len() == seq_len {
                Some(i + seq_len)
            } else {
                None
            }
        }).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
    fn test_part1(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(part1(input), expected);
    }

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 23)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 23)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)]
    fn test_part2(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(part2(input), expected);
    }
}
