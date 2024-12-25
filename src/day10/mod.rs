use std::collections::BTreeSet;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 10;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let input = input.as_bytes();
        let width = input.iter().position(|&b| b == b'\n').unwrap();
        let stride = width + 1;
        // let height = (input.len() + 1) / stride;

        memchr::memchr_iter(b'0', input)
            .map(|pos| {
                let mut map = BTreeSet::new();
                find_trail(input, stride, pos, 1, &mut map);
                map.len()
            })
            .sum()
    }
}

impl SolutionGold<usize, usize> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> usize {
        let input = input.as_bytes();
        let width = input.iter().position(|&b| b == b'\n').unwrap();
        let stride = width + 1;
        // let height = (input.len() + 1) / stride;

        memchr::memchr_iter(b'9', input)
            .map(|pos| find_trail_2(input, stride, pos, 9))
            .sum()
    }
}

fn find_trail(grid: &[u8], stride: usize, index: usize, number: u8, results: &mut BTreeSet<usize>) {
    let x = index % stride;
    let y = index / stride;
    let w = stride - 1;
    let h = (grid.len() + 1) / stride;

    if number == 10 {
        results.insert(index);
    }

    if x < w - 1 && grid[index + 1] == number + b'0' {
        find_trail(grid, stride, index + 1, number + 1, results);
    }
    if x >= 1 && grid[index - 1] == number + b'0' {
        find_trail(grid, stride, index - 1, number + 1, results);
    }
    if y < h - 1 && grid[index + stride] == number + b'0' {
        find_trail(grid, stride, index + stride, number + 1, results);
    }
    if y >= 1 && grid[index - stride] == number + b'0' {
        find_trail(grid, stride, index - stride, number + 1, results);
    }
}

fn find_trail_2(grid: &[u8], stride: usize, index: usize, number: u8) -> usize {
    let x = index % stride;
    let y = index / stride;
    let w = stride - 1;
    let h = (grid.len() + 1) / stride;

    if number == 0 {
        return 1;
    }

    let mut count = 0;

    if x < w - 1 && grid[index + 1] == number + b'0' - 1 {
        count += find_trail_2(grid, stride, index + 1, number - 1);
    }
    if x >= 1 && grid[index - 1] == number + b'0' - 1 {
        count += find_trail_2(grid, stride, index - 1, number - 1);
    }
    if y < h - 1 && grid[index + stride] == number + b'0' - 1 {
        count += find_trail_2(grid, stride, index + stride, number - 1);
    }
    if y >= 1 && grid[index - stride] == number + b'0' - 1 {
        count += find_trail_2(grid, stride, index - stride, number - 1);
    }

    count
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(36, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(688, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(81, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(1459, output);
}
