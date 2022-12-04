fn main() {
    const PUZZLE_DATA: &str = include_str!("day04/puzzle.txt");
    println!("Part1: {}", part1(PUZZLE_DATA));
    println!("Part2: {}", part2(PUZZLE_DATA));
}

fn part1(data: &str) -> usize {
    data.lines()
        .map(|s| {
            let (left, right) = s.split_once(',').unwrap();

            let (a, b) = left.split_once('-').unwrap();
            let (x, y) = right.split_once('-').unwrap();

            [
                a.parse::<u32>().unwrap(),
                b.parse::<u32>().unwrap(),
                x.parse::<u32>().unwrap(),
                y.parse::<u32>().unwrap(),
            ]
        })
        .filter(|&[a, b, c, d]: &[u32; 4]| ((a <= c) && (b >= d)) || ((a >= c) && (b <= d)))
        .count()
}

fn part2(data: &str) -> usize {
    data.lines()
        .map(|s| {
            let (left, right) = s.split_once(',').unwrap();

            let (a, b) = left.split_once('-').unwrap();
            let (x, y) = right.split_once('-').unwrap();

            [
                a.parse::<u32>().unwrap(),
                b.parse::<u32>().unwrap(),
                x.parse::<u32>().unwrap(),
                y.parse::<u32>().unwrap(),
            ]
        })
        .filter(|&[a, b, c, d]: &[u32; 4]| (b >= c && b <= d) || (d >= a && d <= b))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = include_str!("day04/sample.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), 4);
    }
}
