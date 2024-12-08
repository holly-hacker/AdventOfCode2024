use std::collections::HashSet;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 8;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let input = input.as_bytes();
        let width = input.iter().position(|&b| b == b'\n').unwrap();
        let stride = width + 1;
        let height = (input.len() + 1) / stride;

        let antenna_names = input
            .iter()
            .filter(|&&b| b != b'.' && b != b'\n')
            .copied()
            .collect::<HashSet<_>>();

        let mut overlaps = HashSet::<(isize, isize)>::new();
        for chr in antenna_names {
            let positions = input
                .iter()
                .enumerate()
                .filter(|(_, &b)| b == chr)
                .map(|(i, _)| i)
                .collect::<Vec<_>>();

            // pick any 2 positions
            for i in 0..positions.len() {
                for j in (i + 1)..positions.len() {
                    let i_x = (positions[i] % stride) as isize;
                    let i_y = (positions[i] / stride) as isize;

                    let j_x = (positions[j] % stride) as isize;
                    let j_y = (positions[j] / stride) as isize;

                    let mut anti_freq1_x = i_x.min(j_x) - (i_x.abs_diff(j_x) as isize);
                    let mut anti_freq2_x = i_x.max(j_x) + (i_x.abs_diff(j_x) as isize);

                    if i_x > j_x {
                        std::mem::swap(&mut anti_freq1_x, &mut anti_freq2_x);
                    }

                    let anti_freq1_y = i_y - (i_y.abs_diff(j_y) as isize);
                    let anti_freq2_y = j_y + (i_y.abs_diff(j_y) as isize);

                    if (anti_freq1_x >= 0 && anti_freq1_x < width as isize)
                        && (anti_freq1_y >= 0 && anti_freq1_y < height as isize)
                    {
                        overlaps.insert((anti_freq1_x, anti_freq1_y));
                    }
                    if (anti_freq2_x >= 0 && anti_freq2_x < width as isize)
                        && (anti_freq2_y >= 0 && anti_freq2_y < height as isize)
                    {
                        overlaps.insert((anti_freq2_x, anti_freq2_y));
                    }
                }
            }
        }

        overlaps.len()
    }
}

impl SolutionGold<usize, usize> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> usize {
        let input = input.as_bytes();
        let width = input.iter().position(|&b| b == b'\n').unwrap();
        let stride = width + 1;
        let height = (input.len() + 1) / stride;

        let antenna_names = input
            .iter()
            .filter(|&&b| b != b'.' && b != b'\n')
            .copied()
            .collect::<HashSet<_>>();

        let mut overlaps = HashSet::<(isize, isize)>::new();
        for chr in antenna_names {
            let positions = input
                .iter()
                .enumerate()
                .filter(|(_, &b)| b == chr)
                .map(|(i, _)| i)
                .collect::<Vec<_>>();

            // pick any 2 positions
            for i in 0..positions.len() {
                for j in (i + 1)..positions.len() {
                    let i_x = (positions[i] % stride) as isize;
                    let i_y = (positions[i] / stride) as isize;

                    let j_x = (positions[j] % stride) as isize;
                    let j_y = (positions[j] / stride) as isize;

                    let direction_x = j_x - i_x;
                    let direction_y = j_y - i_y;

                    let mut loop_iter = 0;
                    loop {
                        let anti_freq1_x = i_x - direction_x * loop_iter;
                        let anti_freq1_y = i_y - direction_y * loop_iter;
                        let anti_freq2_x = j_x + direction_x * loop_iter;
                        let anti_freq2_y = j_y + direction_y * loop_iter;

                        let mut any_insert = false;
                        if (anti_freq1_x >= 0 && anti_freq1_x < width as isize)
                            && (anti_freq1_y >= 0 && anti_freq1_y < height as isize)
                        {
                            overlaps.insert((anti_freq1_x, anti_freq1_y));
                            any_insert = true;
                        }
                        if (anti_freq2_x >= 0 && anti_freq2_x < width as isize)
                            && (anti_freq2_y >= 0 && anti_freq2_y < height as isize)
                        {
                            overlaps.insert((anti_freq2_x, anti_freq2_y));
                            any_insert = true;
                        }

                        if !any_insert {
                            break;
                        }
                        loop_iter += 1;
                    }
                }
            }
        }

        overlaps.len()
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(14, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(214, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(34, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(809, output);
}
