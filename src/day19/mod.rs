use std::collections::HashMap;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 19;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let (patterns, designs) = input.split_once("\n\n").unwrap();

        let mut patterns = patterns.split(", ").collect::<Vec<_>>();

        patterns.sort_by_key(|p| p.len());

        let mut history = HashMap::<_, _, rustc_hash::FxBuildHasher>::default();

        designs
            .lines()
            .filter(|design| check_can_be_made(design, &patterns, &mut history))
            .count()
    }
}

fn check_can_be_made<'a>(
    design: &'a str,
    patterns: &[&str],
    history: &mut HashMap<&'a str, bool, rustc_hash::FxBuildHasher>,
) -> bool {
    if design.is_empty() {
        return true;
    }

    if let Some(success) = history.get(design) {
        return *success;
    }

    let mut success = false;
    for &pattern in patterns {
        if pattern.len() > design.len() {
            continue;
        }

        if &design[..pattern.len()] != pattern {
            continue;
        }

        if check_can_be_made(&design[pattern.len()..], patterns, history) {
            success = true;
            break;
        }
    }

    history.insert(design, success);

    success
}

impl SolutionGold<usize, usize> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> usize {
        let (patterns, designs) = input.split_once("\n\n").unwrap();

        let patterns = patterns.split(", ").collect::<Vec<_>>();

        let mut history = HashMap::<_, _, rustc_hash::FxBuildHasher>::default();

        designs
            .lines()
            .map(|design| check_can_be_made_gold(design, &patterns, &mut history))
            .sum()
    }
}

fn check_can_be_made_gold<'a>(
    design: &'a str,
    patterns: &[&str],
    history: &mut HashMap<&'a str, usize, rustc_hash::FxBuildHasher>,
) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(success) = history.get(design) {
        return *success;
    }

    let mut success_count = 0;

    for &pattern in patterns {
        if pattern.len() > design.len() {
            continue;
        }

        if &design[..pattern.len()] != pattern {
            continue;
        }

        success_count += check_can_be_made_gold(&design[pattern.len()..], patterns, history);
    }

    history.insert(design, success_count);

    success_count
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(6, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(290, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(16, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(712058625427487, output);
}
