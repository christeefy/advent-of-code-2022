fn main() {
    const PUZZLE_DATA: &str = include_str!("day01/puzzle.txt");

    println!(
        "Part1: {}\nPart2: {}",
        get_largest_calories(PUZZLE_DATA),
        get_total_top_n_calories(PUZZLE_DATA, 3)
    );
}

pub fn get_largest_calories(string: &str) -> u32 {
    *get_each_elfs_calories(string)
        .iter()
        .max()
        .unwrap_or_else(|| panic!("Empty iterator found"))
}

pub fn get_total_top_n_calories(string: &str, top_n: usize) -> u32 {
    let mut calories = get_each_elfs_calories(string);
    calories.sort_by(|a, b| b.cmp(a));
    calories[..top_n].iter().sum()
}

fn get_each_elfs_calories(string: &str) -> Vec<u32> {
    string
        .split("\n\n")
        .map(|calories_str| {
            calories_str
                .split('\n')
                .map(|s| {
                    s.parse::<u32>()
                        .unwrap_or_else(|_| panic!("Failed to parse {s} as u32"))
                })
                .sum::<u32>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = include_str!("day01/sample.txt");

    #[test]
    fn test_get_largest_calorie() {
        assert_eq!(get_largest_calories(DATA), 24000)
    }

    #[test]
    fn test_get_total_top_n_calories() {
        assert_eq!(get_total_top_n_calories(DATA, 3), 45000);
    }
}
