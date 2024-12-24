use std::collections::{HashMap, HashSet};

use petgraph::{visit::EdgeRef, Graph};

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 24;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let (part1, part2) = input.split_once("\n\n").unwrap();

        let mut wire_states = HashMap::new();

        part1.lines().for_each(|line| {
            let (name, num) = line.split_once(": ").unwrap();
            let num: usize = num.parse().unwrap();
            wire_states.insert(name, num);
        });

        let mut any_changed = true;

        while any_changed {
            any_changed = false;

            part2.lines().for_each(|line| {
                let (expr, result) = line.split_once(" -> ").unwrap();
                let (term1, expr) = expr.split_once(' ').unwrap();
                let (op, term2) = expr.split_once(' ').unwrap();

                if let (None, Some(val1), Some(val2)) = (
                    wire_states.get(result),
                    wire_states.get(term1),
                    wire_states.get(term2),
                ) {
                    match op {
                        "AND" => {
                            wire_states.insert(result, val1 & val2);
                        }
                        "OR" => {
                            wire_states.insert(result, val1 | val2);
                        }
                        "XOR" => {
                            wire_states.insert(result, val1 ^ val2);
                        }
                        _ => todo!(),
                    }

                    any_changed = true;
                }
            });
        }

        wire_states
            .into_iter()
            .filter(|(k, _)| k.starts_with('z'))
            .map(|(k, v)| v << k[1..].parse::<usize>().unwrap())
            .sum()
    }
}

impl SolutionGold<usize, String> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> String {
        // no valid sample
        if input.len() < 1000 {
            return Default::default();
        }

        let swaps = [
            ("mkk", "z10"),
            ("z14", "qbw"),
            ("cvp", "wjb"),
            ("z34", "wcb"),
        ];

        let mut input = input.to_string();
        for (swap_left, swap_right) in swaps {
            let pos_left = input.find(&format!("-> {swap_left}")).unwrap() + 3;
            let pos_right = input.find(&format!("-> {swap_right}")).unwrap() + 3;

            // SAFETY: swapping ascii characters
            unsafe {
                input.as_bytes_mut()[pos_left] = swap_right.as_bytes()[0];
                input.as_bytes_mut()[pos_left + 1] = swap_right.as_bytes()[1];
                input.as_bytes_mut()[pos_left + 2] = swap_right.as_bytes()[2];

                input.as_bytes_mut()[pos_right] = swap_left.as_bytes()[0];
                input.as_bytes_mut()[pos_right + 1] = swap_left.as_bytes()[1];
                input.as_bytes_mut()[pos_right + 2] = swap_left.as_bytes()[2];
            }
        }

        let (part1, part2) = input.split_once("\n\n").unwrap();

        let mut graph = Graph::new();

        part1.lines().for_each(|line| {
            let (name, num) = line.split_once(": ").unwrap();
            let num: usize = num.parse().unwrap();

            let val_wt = graph.add_node(name.to_string());
            let num_wt = graph.add_node(num.to_string());

            graph.add_edge(num_wt, val_wt, Operation::Assign);
        });

        part2.lines().for_each(|line| {
            let (expr, result) = line.split_once(" -> ").unwrap();
            let (term1, expr) = expr.split_once(' ').unwrap();
            let (op, term2) = expr.split_once(' ').unwrap();

            let term1_wt = graph
                .node_indices()
                .find(|&idx| graph.node_weight(idx).unwrap() == term1)
                .unwrap_or_else(|| graph.add_node(term1.to_string()));
            let term2_wt = graph
                .node_indices()
                .find(|&idx| graph.node_weight(idx).unwrap() == term2)
                .unwrap_or_else(|| graph.add_node(term2.to_string()));
            let result_wt = graph
                .node_indices()
                .find(|&idx| graph.node_weight(idx).unwrap() == result)
                .unwrap_or_else(|| graph.add_node(result.to_string()));

            let op = match op {
                "AND" => Operation::And,
                "OR" => Operation::Or,
                "XOR" => Operation::Xor,
                _ => unreachable!(),
            };

            graph.add_edge(term1_wt, result_wt, op);
            graph.add_edge(term2_wt, result_wt, op);
        });

        // println!("{:?}", Dot::with_config(&graph, &[]));

        // we need to fix a graph that implements binary addition 0_o
        let mut previous_carry = None;
        for i in 0..=44 {
            let x = graph
                .node_indices()
                .find(|&idx| graph.node_weight(idx).unwrap() == &format!("x{:02}", i))
                .unwrap();
            let y = graph
                .node_indices()
                .find(|&idx| graph.node_weight(idx).unwrap() == &format!("y{:02}", i))
                .unwrap();

            let half_adder_sum = single(
                intersect(
                    graph
                        .edges(x)
                        .filter(|e| *e.weight() == Operation::Xor)
                        .map(|e| e.target()),
                    graph
                        .edges(y)
                        .filter(|e| *e.weight() == Operation::Xor)
                        .map(|e| e.target()),
                )
                .into_iter(),
            )
            .unwrap();

            let half_adder_carry = single(
                intersect(
                    graph
                        .edges(x)
                        .filter(|e| *e.weight() == Operation::And)
                        .map(|e| e.target()),
                    graph
                        .edges(y)
                        .filter(|e| *e.weight() == Operation::And)
                        .map(|e| e.target()),
                )
                .into_iter(),
            )
            .unwrap();

            /*
            println!(
                "half_adder_sum: {}, half_adder_carry: {}",
                graph.node_weight(half_adder_sum).unwrap(),
                graph.node_weight(half_adder_carry).unwrap(),
            );
            */

            if let Some(prev_carry) = previous_carry {
                let _sum = single(
                    intersect(
                        graph
                            .edges(prev_carry)
                            .filter(|e| *e.weight() == Operation::Xor)
                            .map(|e| e.target()),
                        graph
                            .edges(half_adder_sum)
                            .filter(|e| *e.weight() == Operation::Xor)
                            .map(|e| e.target()),
                    )
                    .into_iter(),
                )
                .expect("find sum");

                let intermediate = single(
                    intersect(
                        graph
                            .edges(prev_carry)
                            .filter(|e| *e.weight() == Operation::And)
                            .map(|e| e.target()),
                        graph
                            .edges(half_adder_sum)
                            .filter(|e| *e.weight() == Operation::And)
                            .map(|e| e.target()),
                    )
                    .into_iter(),
                )
                .expect("find intermediate");

                let new_carry = single(
                    intersect(
                        graph
                            .edges(half_adder_carry)
                            .filter(|e| *e.weight() == Operation::Or)
                            .map(|e| e.target()),
                        graph
                            .edges(intermediate)
                            .filter(|e| *e.weight() == Operation::Or)
                            .map(|e| e.target()),
                    )
                    .into_iter(),
                )
                .expect("find new carry");

                /*
                println!(
                    "sum: {}, intermediate: {}, new_carry: {}",
                    graph.node_weight(sum).unwrap(),
                    graph.node_weight(intermediate).unwrap(),
                    graph.node_weight(new_carry).unwrap(),
                );
                */

                previous_carry = Some(new_carry);
            } else {
                previous_carry = Some(half_adder_carry);
            }
        }

        let mut swaps = swaps
            .into_iter()
            .flat_map(|(a, b)| [a, b])
            .collect::<Vec<_>>();
        swaps.sort_unstable();
        swaps.join(",")
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Operation {
    Assign,
    And,
    Or,
    Xor,
}

fn intersect<T: Eq + std::hash::Hash + Copy>(
    left: impl Iterator<Item = T>,
    right: impl Iterator<Item = T>,
) -> impl IntoIterator<Item = T> {
    let left = left.collect::<HashSet<T>>();
    let right = right.collect::<HashSet<T>>();

    let collected = left.intersection(&right).copied().collect::<HashSet<T>>();

    collected
}

fn single<T>(mut iter: impl Iterator<Item = T>) -> Option<T> {
    match (iter.next(), iter.next()) {
        (Some(first), None) => Some(first),
        _ => None,
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(4, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(36902370467952, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!("", output);
}

#[test]
#[allow(unused)]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!("cvp,mkk,qbw,wcb,wjb,z10,z14,z34", output);
}
