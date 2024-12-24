use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 23;
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
        todo!()
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
    assert_eq!("co,de,ka,ta", output);
}

#[test]
#[allow(unused)]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!("hf,hz,lb,lm,ls,my,ps,qu,ra,uc,vi,xz,yv", output);
}
