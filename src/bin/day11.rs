use std::collections::VecDeque;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, digit1, multispace0, multispace1, one_of},
    combinator::map,
    multi::{many1, separated_list0},
    sequence::{delimited, separated_pair},
    IResult,
};

fn main() {
    const PUZZLE_DATA: &str = include_str!("day11/puzzle.txt");
    println!("Part1: {}", part1(PUZZLE_DATA));
    println!("Part2: {}", part2(PUZZLE_DATA));
}

fn part1(data: &str) -> u64 {
    let (_, mut monkeys) = parse(data).unwrap();
    process(&mut monkeys, 20, true)
}

fn part2(data: &str) -> u64 {
    let (_, mut monkeys) = parse(data).unwrap();
    process(&mut monkeys, 10_000, false)
}

fn process(monkeys: &mut Vec<Monkey>, n_rounds: u64, worry_level_decerase: bool) -> u64 {
    let divisor_prod: u64 = monkeys
        .iter()
        .map(|monkey| monkey.decision_data.divisor)
        .product();

    for _ in 0..n_rounds {
        for src in 0..monkeys.len() {
            for _ in 0..monkeys[src].items.len() {
                let src_monkey = &mut monkeys[src];
                let (item, target) =
                    src_monkey.inspect_item_and_throw_to(worry_level_decerase, divisor_prod);
                let target_monkey = &mut monkeys[target];
                target_monkey.receive_item(item);
            }
        }
    }

    let mut num_items_inspected: Vec<_> = monkeys.iter().map(|monkey| monkey.n_inspected).collect();
    num_items_inspected.sort_by(|a, b| b.cmp(a));
    num_items_inspected[0] * num_items_inspected[1]
}

#[derive(Debug)]
enum Operation {
    Add(u64),
    Mult(u64),
    Square,
}

#[derive(Debug)]
struct DecisionData {
    divisor: u64,
    target_if_true: usize,
    target_if_false: usize,
}

impl DecisionData {
    fn decide(&self, x: u64) -> usize {
        if (x % self.divisor) == 0 {
            self.target_if_true
        } else {
            self.target_if_false
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    n_inspected: u64,
    operation: Operation,
    decision_data: DecisionData,
}

impl Monkey {
    fn inspect_item_and_throw_to(
        &mut self,
        worry_level_decrease: bool,
        divisor_prod: u64,
    ) -> (u64, usize) {
        self.n_inspected += 1;

        let mut item = self.items.pop_front().unwrap();

        // Worry increases
        item = match self.operation {
            Operation::Add(by) => item + by,
            Operation::Mult(by) => item * by,
            Operation::Square => item.pow(2),
        } % divisor_prod;

        // Worry decreases
        if worry_level_decrease {
            item /= 3;
        }

        // Compute which monkey to throw item to
        let target = self.decision_data.decide(item);

        (item, target)
    }

    fn receive_item(&mut self, item: u64) {
        self.items.push_back(item);
    }
}

fn starting_items(s: &str) -> IResult<&str, VecDeque<u64>> {
    delimited(
        tag("Starting items: "),
        map(separated_list0(tag(", "), complete::u64), VecDeque::from),
        multispace1,
    )(s)
}

fn operation(s: &str) -> IResult<&str, Operation> {
    delimited(
        tag("Operation: new = old "),
        map(
            separated_pair(one_of("+*"), complete::char(' '), alt((digit1, tag("old")))),
            |(operator, operand)| match (operator, operand) {
                ('+', operand) => Operation::Add(operand.parse().unwrap()),
                ('*', "old") => Operation::Square,
                ('*', operand) => Operation::Mult(operand.parse().unwrap()),
                _ => panic!("Parsed invalid operation"),
            },
        ),
        multispace1,
    )(s)
}

fn decision_fn(s: &str) -> IResult<&str, DecisionData> {
    let (s, divisor) = delimited(tag("Test: divisible by "), complete::u64, multispace1)(s)?;
    let (s, target_if_true) =
        delimited(tag("If true: throw to monkey "), complete::u64, multispace1)(s)?;
    let (s, target_if_false) = delimited(
        tag("If false: throw to monkey "),
        complete::u64,
        multispace0,
    )(s)?;

    Ok((
        s,
        DecisionData {
            divisor,
            target_if_true: target_if_true as usize,
            target_if_false: target_if_false as usize,
        },
    ))
}

fn monkey(s: &str) -> IResult<&str, Monkey> {
    let (s, _) = separated_pair(tag("Monkey "), digit1, complete::char(':'))(s)?;
    let (s, _) = multispace1(s)?;
    let (s, starting_items) = starting_items(s)?;
    let (s, operation) = operation(s)?;
    let (s, decision_data) = decision_fn(s)?;

    Ok((
        s,
        Monkey {
            items: starting_items,
            n_inspected: 0,
            operation,
            decision_data,
        },
    ))
}

fn parse(s: &str) -> IResult<&str, Vec<Monkey>> {
    many1(monkey)(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = include_str!("day11/sample.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 10605);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), 2713310158);
    }
}
