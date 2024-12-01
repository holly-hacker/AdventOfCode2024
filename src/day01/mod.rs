use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 1;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let mut nums1 = vec![];
        let mut nums2 = vec![];
        input
            .lines()
            .map(|line| line.split_once("   ").unwrap())
            .for_each(|(left, right)| {
                nums1.push(left.parse::<usize>().unwrap());
                nums2.push(right.parse::<usize>().unwrap());
            });

        nums1.sort();
        nums2.sort();

        nums1
            .into_iter()
            .zip(nums2.into_iter())
            .map(|(a, b)| a.abs_diff(b))
            .sum()
    }
}

impl SolutionGold<usize, usize> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> usize {
        let mut nums1 = vec![];
        let mut nums2 = vec![];
        input
            .lines()
            .map(|line| line.split_once("   ").unwrap())
            .for_each(|(left, right)| {
                nums1.push(left.parse::<usize>().unwrap());
                nums2.push(right.parse::<usize>().unwrap());
            });

        nums1
            .into_iter()
            .map(|n| nums2.iter().filter(|m| **m == n).count() * n)
            .sum()
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(11, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(1646452, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(31, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(23609874, output);
}
