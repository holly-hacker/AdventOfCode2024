use utils::fast_parse_int;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 22;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        input
            .lines()
            .map(fast_parse_int)
            .map(|num| {
                let mut num = num;
                for _ in 0..2000 {
                    num = num ^ (num * 64);
                    num %= 16777216;
                    num = num ^ (num / 32);
                    num %= 16777216;
                    num = num ^ (num * 2048);
                    num %= 16777216;
                }

                num
            })
            .sum()
    }
}

impl SolutionGold<usize, usize> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> usize {
        let mut highest_found = 0;

        let hashes_list = input
            .lines()
            .map(fast_parse_int)
            .map(|mut num| {
                let mut acc: Vec<(usize, isize)> = vec![];
                let mut prev_num = 0;
                (0..2000).for_each(|_| {
                    num = num ^ (num * 64);
                    num %= 16777216;
                    num = num ^ (num / 32);
                    num %= 16777216;
                    num = num ^ (num * 2048);
                    num %= 16777216;

                    acc.push((num, (num % 10) as isize - (prev_num % 10) as isize));
                    prev_num = num;
                });
                acc[0].1 = -10; // ensure first does not match
                acc
            })
            .collect::<Vec<_>>();

        for n0 in -9..=9 {
            dbg!(n0);
            for n1 in -9..=9 {
                for n2 in -9..=9 {
                    for n3 in -9..=9 {
                        let changes: [isize; 4] = [n0, n1, n2, n3];

                        let mut sum = 0;
                        for hashes in hashes_list.iter() {
                            let idx = hashes.windows(4).position(|window| {
                                window[0].1 == changes[0]
                                    && window[1].1 == changes[1]
                                    && window[2].1 == changes[2]
                                    && window[3].1 == changes[3]
                            });

                            if let Some(idx) = idx {
                                sum += hashes[idx + 3].0 % 10;
                            }
                        }

                        if sum > highest_found {
                            println!("Found: {} with {:?}", sum, changes);
                        }
                        highest_found = highest_found.max(sum);
                    }
                }
            }
        }

        highest_found
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(37327623, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(18941802053, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(23, output);
}

// #[test]
#[allow(unused)]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(2218, output);
}
