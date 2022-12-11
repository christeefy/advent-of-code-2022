use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::map,
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

fn main() {
    const PUZZLE_DATA: &str = include_str!("day10/puzzle.txt");
    println!("Part1: {}", part1(PUZZLE_DATA));
    println!("Part2:\n{}", part2(PUZZLE_DATA));
}

fn part1(data: &str) -> i32 {
    let (_, instructions) = parse(data).unwrap();
    let mut clock_circuit = ClockCircuit::new();

    for instruction in instructions {
        clock_circuit.process(instruction);
    }

    clock_circuit.signal_strength
}

fn part2(data: &str) -> String {
    let (_, instructions) = parse(data).unwrap();
    let mut clock_circuit = ClockCircuit::new();

    for instruction in instructions {
        clock_circuit.process(instruction);
    }

    clock_circuit
        .display_chars
        .iter()
        .map(|x| x.iter().collect::<String>())
        .join("\n")
}

#[derive(Debug)]
enum CpuInstruction {
    AddX(i32),
    NoOp,
}

#[derive(Debug)]
struct ClockCircuit {
    register_val: i32,
    cycle_count: u32,
    signal_strength: i32,
    display_chars: [[char; 40]; 6],
}

impl ClockCircuit {
    fn new() -> Self {
        Self {
            register_val: 1,
            cycle_count: 0,
            signal_strength: 0,
            display_chars: [['.'; 40]; 6],
        }
    }

    fn process(&mut self, instruction: CpuInstruction) {
        match instruction {
            CpuInstruction::NoOp => self.tick(1),
            CpuInstruction::AddX(val) => {
                self.tick(2);
                self.register_val += val;
            }
        }
    }

    fn tick(&mut self, n: u32) {
        for _ in 0..n {
            self.cycle_count += 1;
            self.update_display();
            if self.cycle_count % 40 == 20 {
                // dbg!(&self, (self.cycle_count as i32) * self.register);
                self.signal_strength += (self.cycle_count as i32) * self.register_val;
            }
        }
    }

    fn update_display(&mut self) {
        const SCREEN_WIDTH: usize = 40;
        if ((((self.cycle_count - 1) % SCREEN_WIDTH as u32) as i32) - self.register_val).abs() <= 1
        {
            self.display_chars[(self.cycle_count as usize) / SCREEN_WIDTH]
                [(self.cycle_count as usize - 1) % SCREEN_WIDTH] = '#';
        }
    }
}

fn addx(s: &str) -> IResult<&str, CpuInstruction> {
    map(preceded(tag("addx "), complete::i32), |num| {
        CpuInstruction::AddX(num)
    })(s)
}

fn noop(s: &str) -> IResult<&str, CpuInstruction> {
    map(tag("noop"), |_| CpuInstruction::NoOp)(s)
}

fn parse(s: &str) -> IResult<&str, Vec<CpuInstruction>> {
    separated_list1(newline, alt((addx, noop)))(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = include_str!("day10/sample.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 13140);
    }

    #[test]
    fn test_part2() {
        let expected = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......###.
#######.......#######.......#######....#"
            .to_owned();

        let output = part2(DATA);

        println!("Output:\n{output}");
        println!("Expected:\n{expected}");

        assert_eq!(output, expected);
    }
}
