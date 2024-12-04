use utils::fast_parse_int_from_bytes;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 1;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let input = input.as_bytes();

        let line_len = input.iter().position(|&c| c == b'\n').unwrap();
        let stride = line_len + 1;
        let number_len = (line_len - 3) / 2;
        let line_count = (input.len() + 1) / stride;

        let mut nums1 = Vec::with_capacity(1000);
        let mut nums2 = Vec::with_capacity(1000);

        (0..line_count).for_each(|i| {
            let line_start = stride * i;
            let num2_start = line_start + number_len + 3;
            let num1 = fast_parse_int_from_bytes(&input[line_start..line_start + number_len]);
            let num2 = fast_parse_int_from_bytes(&input[num2_start..num2_start + number_len]);

            nums1.push(num1);
            nums2.push(num2);
        });

        nums1.sort();
        nums2.sort();

        nums1
            .into_iter()
            .zip(nums2)
            .map(|(a, b)| a.abs_diff(b))
            .sum()
    }
}

impl SolutionGold<usize, usize> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> usize {
        return Self::calculate_gold_opt(input);

        #[allow(unreachable_code)]
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

impl Day {
    fn calculate_gold_opt(input: &str) -> usize {
        let input = input.as_bytes();

        let line_len = input.iter().position(|&c| c == b'\n').unwrap();
        let stride = line_len + 1;
        let number_len = (line_len - 3) / 2;
        let line_count = (input.len() + 1) / stride;

        let mut nums1 = Vec::with_capacity(1000);
        let mut nums2 = Vec::with_capacity(1000);

        (0..line_count).for_each(|i| {
            let line_start = stride * i;
            let num2_start = line_start + number_len + 3;
            let num1 = fast_parse_int_from_bytes(&input[line_start..line_start + number_len]);
            let num2 = fast_parse_int_from_bytes(&input[num2_start..num2_start + number_len]);

            nums1.push(num1);
            nums2.push(num2);
        });

        // convert nums2 into a pseudo-hashmap
        nums2.sort();
        let mut nums2_counts = vec![];
        let mut current_num = nums2[0];
        let mut current_count = 0;
        for num in nums2.into_iter() {
            if num == current_num {
                current_count += 1;
            } else {
                nums2_counts.push((current_num, current_count));
                current_num = num;
                current_count = 1;
            }
        }
        if current_count > 0 {
            nums2_counts.push((current_num, current_count));
        }

        nums1
            .into_iter()
            .map(|num| {
                num * nums2_counts
                    .binary_search_by(|&(k, _)| k.cmp(&num))
                    .map(|pos| nums2_counts[pos].1)
                    .unwrap_or_default()
            })
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
