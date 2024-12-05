use utils::fast_parse_int;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 5;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let (rules, updates) = input.split_once("\n\n").unwrap();
        let rules: Vec<(usize, usize)> = rules
            .lines()
            .map(|line| {
                let (l, r) = line.split_once('|').unwrap();
                (fast_parse_int(l), fast_parse_int(r))
            })
            .collect();
        let updates: Vec<Vec<usize>> = updates
            .lines()
            .map(|line| line.split(',').map(fast_parse_int).collect())
            .collect();

        updates
            .into_iter()
            .filter(|u| {
                rules.iter().all(|r| {
                    match (
                        u.iter().position(|&x| x == r.0),
                        u.iter().position(|&x| x == r.1),
                    ) {
                        (Some(p0), Some(p1)) => p0 <= p1,
                        _ => true,
                    }
                })
            })
            .map(|u| u[u.len() / 2])
            .sum()
    }
}

impl SolutionGold<usize, usize> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> usize {
        let (rules, updates) = input.split_once("\n\n").unwrap();
        let rules: Vec<(usize, usize)> = rules
            .lines()
            .map(|line| {
                let (l, r) = line.split_once('|').unwrap();
                (fast_parse_int(l), fast_parse_int(r))
            })
            .collect();
        let updates: Vec<Vec<usize>> = updates
            .lines()
            .map(|line| line.split(',').map(fast_parse_int).collect())
            .collect();

        updates
            .into_iter()
            .filter(|u| {
                // inverted
                !rules.iter().all(|r| {
                    match (
                        u.iter().position(|&x| x == r.0),
                        u.iter().position(|&x| x == r.1),
                    ) {
                        (Some(p0), Some(p1)) => p0 <= p1,
                        _ => true,
                    }
                })
            })
            .map(|mut u| {
                let mut correct = false;

                while !correct {
                    rules.iter().for_each(|r| {
                        let a = u.iter().position(|&x| x == r.0);
                        let b = u.iter().position(|&x| x == r.1);

                        if let (Some(a), Some(b)) = (a, b) {
                            if a > b {
                                u.swap(a, b);
                            }
                        }
                    });

                    correct = rules.iter().all(|r| {
                        match (
                            u.iter().position(|&x| x == r.0),
                            u.iter().position(|&x| x == r.1),
                        ) {
                            (Some(p0), Some(p1)) => p0 <= p1,
                            _ => true,
                        }
                    });
                }

                u
            })
            .map(|u| u[u.len() / 2])
            .sum()
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(143, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(4185, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(123, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(4480, output);
}
