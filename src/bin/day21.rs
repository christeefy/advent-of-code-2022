use petgraph::prelude::DiGraphMap;
use petgraph::visit::Topo;
use petgraph::visit::Walker;
use std::collections::BTreeMap;

use petgraph::dot::{Config, Dot};

use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::{
    branch::alt,
    character::complete,
    character::complete::{alpha1, one_of},
    sequence::separated_pair,
    IResult,
};

fn main() {
    const PUZZLE_DATA: &str = include_str!("day21/puzzle.txt");
    println!("Part1: {}", part1(PUZZLE_DATA));
    // println!("Part2: {}", part2(PUZZLE_DATA));
}

fn part1(data: &str) -> i64 {
    let (_, (map, graph)) = parse(data).unwrap();
    let state = build_state(&map, &graph, false);
    *state.get("root").unwrap()
}

fn part2(data: &str) -> i64 {
    let (_, (map, graph)) = parse(data).unwrap();
    let state = build_state(&map, &graph, true);
    let reverse_graph = create_reverse_graph(&map, &graph, &state);

    // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    todo!()
}

fn build_state<'a>(
    map: &MonkeyMap<'a>,
    graph: &MonkeyGraph<'a>,
    is_part2: bool,
) -> BTreeMap<&'a str, i64> {
    let mut state = BTreeMap::new();

    for id in Topo::new(graph).iter(graph) {
        if is_part2 && id == "humn" {
            continue;
        }

        if is_part2 && id == "root" {
            if let &Operation::Pairwise { left, right, .. } = &map.get(id).unwrap().op {
                match (state.get(left), state.get(right)) {
                    (Some(&val), None) => state.insert(right, val),
                    (None, Some(&val)) => state.insert(left, val),
                    _ => unreachable!(),
                };
            }
            continue;
        }

        let result = map.get(id).unwrap().op.calculate(&state);
        state.insert(id, result);
    }

    state
}

// fn update_state_reverse_graph<'a>(
//     state: &mut BTreeMap<&'a str, i64>,
//     map: &MonkeyMap<'a>,
//     rev_graph: &MonkeyGraph<'a>,
// ) {

//     for id in Topo::new(rev_graph).iter(rev_graph) {
//         let result = map.get(id).unwrap().op.reverse_calculate(*state.get(id).unwrap(), state);
//         match (state.get())

//     }
//     todo!()
// }

fn create_reverse_graph<'a>(
    map: &MonkeyMap<'a>,
    graph: &MonkeyGraph<'a>,
    state: &BTreeMap<&'a str, i64>,
) -> MonkeyGraph<'a> {
    let edges = Topo::new(graph)
        .iter(graph)
        .filter_map(|id| {
            if let &Operation::Pairwise { left, right, .. } = &map.get(id).unwrap().op {
                match (state.get(left), state.get(right)) {
                    (Some(_), None) => Some(vec![(left, right), (id, right)]),
                    (None, Some(_)) => Some(vec![(right, left), (id, left)]),
                    (Some(_), Some(_)) => None, // A node which was part of the non-humn graph. Simply ignore
                    _ => unreachable!(),
                }
            } else {
                None
            }
        })
        .flatten()
        .collect::<Vec<_>>();

    MonkeyGraph::from_edges(edges)
}

#[derive(Debug)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
enum Operation<'a> {
    Number(i64),
    Pairwise {
        left: &'a str,
        operator: Operator,
        right: &'a str,
    },
}

impl<'a> Operation<'a> {
    fn calculate(&self, state: &BTreeMap<&'a str, i64>) -> i64 {
        match self {
            Self::Number(x) => *x,
            Self::Pairwise {
                left,
                operator,
                right,
            } => {
                if let (Some(left), Some(right)) = (state.get(left), state.get(right)) {
                    match operator {
                        Operator::Add => left + right,
                        Operator::Sub => left - right,
                        Operator::Mul => left * right,
                        Operator::Div => left / right,
                    }
                } else {
                    panic!()
                }
            }
        }
    }

    fn reverse_calculate(&self, root: i64, state: &BTreeMap<&'a str, i64>) -> i64 {
        match self {
            Self::Number(x) => *x,
            Self::Pairwise {
                left,
                operator,
                right,
            } => match (operator, state.get(left), state.get(right)) {
                (Operator::Add, Some(val), None) | (Operator::Add, None, Some(val)) => root - val,
                (Operator::Mul, Some(val), None) | (Operator::Mul, None, Some(val)) => root / val,
                (Operator::Sub, Some(val), None) => val - root,
                (Operator::Sub, None, Some(val)) => val + root,
                (Operator::Div, Some(val), None) => val / root,
                (Operator::Div, None, Some(val)) => val * root,
                _ => panic!(),
            },
        }
    }
}

#[derive(Debug)]
struct Monkey<'a> {
    id: &'a str,
    op: Operation<'a>,
}

type MonkeyMap<'a> = BTreeMap<&'a str, Monkey<'a>>;
type MonkeyGraph<'a> = DiGraphMap<&'a str, ()>;

fn pairwise_operation(input: &str) -> IResult<&str, Operation> {
    map(
        separated_pair(
            alpha1,
            complete::char(' '),
            separated_pair(one_of("+-*/"), complete::char(' '), alpha1),
        ),
        |(left, (operator, right))| Operation::Pairwise {
            left,
            operator: match operator {
                '+' => Operator::Add,
                '-' => Operator::Sub,
                '*' => Operator::Mul,
                '/' => Operator::Div,
                _ => panic!("Invalid operator '{operator}' obtained"),
            },
            right,
        },
    )(input)
}

fn number(input: &str) -> IResult<&str, Operation> {
    map(complete::i64, Operation::Number)(input)
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    // map(
    //     separated_pair(alpha1, tag(": "), alt((number, math_operation))),
    //     |(str, op)| (str.to_owned(), op),
    // )(input)
    map(
        separated_pair(alpha1, tag(": "), alt((number, pairwise_operation))),
        |(id, op)| Monkey { id, op },
    )(input)
}

fn parse(input: &str) -> IResult<&str, (MonkeyMap, MonkeyGraph)> {
    let (input, monkeys) = (separated_list1(complete::newline, monkey))(input)?;

    let edges = monkeys
        .iter()
        .flat_map(|monkey| match monkey.op {
            Operation::Number(_) => vec![],
            Operation::Pairwise { left, right, .. } => vec![(left, monkey.id), (right, monkey.id)],
        })
        .collect::<Vec<_>>();

    let graph = DiGraphMap::from_edges(edges);

    let monkeys = monkeys
        .into_iter()
        .map(|monkey| (monkey.id, monkey))
        .collect();

    Ok((input, (monkeys, graph)))
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = include_str!("day21/sample.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 152);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), 301);
    }
}
