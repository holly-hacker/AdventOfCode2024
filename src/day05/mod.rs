use tinyvec::ArrayVec;
use utils::fast_parse_int;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 5;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let input = input.as_bytes();
        let split_index = memchr::memmem::find(input, b"\n\n").unwrap();
        let rules_slice = &input[..split_index];
        let updates_slice = &input[(split_index + 2)..];

        // all numbers seem to be from 11 to 99 (inclusive)
        let mut rules = [ArrayVec::<[u8; 24]>::new(); 99 - 11 + 1];

        let rule_count = rules_slice.len() / 6;
        (0..rule_count).for_each(|rule_idx| {
            let left =
                (rules_slice[rule_idx * 6] - b'0') * 10 + rules_slice[rule_idx * 6 + 1] - b'0';
            let right =
                (rules_slice[rule_idx * 6 + 3] - b'0') * 10 + rules_slice[rule_idx * 6 + 4] - b'0';
            rules[left as usize - 11].push(right);
        });

        updates_slice
            .split(|&b| b == b'\n')
            .map(|line| {
                let num_count = (line.len() + 1) / 3;
                (0..num_count)
                    .map(|num_i| (line[num_i * 3] - b'0') * 10 + line[num_i * 3 + 1] - b'0')
                    .collect::<ArrayVec<[u8; 23]>>()
            })
            .filter(|update| {
                let mut seen = [false; 99 - 11 + 1];
                (0..update.len()).all(|idx| {
                    let cur = update[idx];
                    let rules = &rules[(cur - 11) as usize];

                    let ok = rules.is_empty() || rules.iter().all(|&r| !seen[r as usize - 11]);

                    seen[cur as usize - 11] = true;

                    ok
                })
            })
            .map(|u| u[u.len() / 2] as usize)
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
