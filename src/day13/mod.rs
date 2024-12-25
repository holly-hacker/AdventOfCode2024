use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 13;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        input
            .split("\n\n")
            .map(|conf| {
                let mut lines = conf.lines();
                let line_1 = lines.next().unwrap().split_once(": ").unwrap().1;
                let line_2 = lines.next().unwrap().split_once(": ").unwrap().1;
                let line_3 = lines.next().unwrap().split_once(": ").unwrap().1;

                let (a_x, a_y) = line_1.split_once(", ").unwrap();
                let (a_x, a_y) = (
                    a_x[2..].parse::<usize>().unwrap(),
                    a_y[2..].parse::<usize>().unwrap(),
                );
                let (b_x, b_y) = line_2.split_once(", ").unwrap();
                let (b_x, b_y) = (
                    b_x[2..].parse::<usize>().unwrap(),
                    b_y[2..].parse::<usize>().unwrap(),
                );
                let (p_x, p_y) = line_3.split_once(", ").unwrap();
                let (p_x, p_y) = (
                    p_x[2..].parse::<usize>().unwrap(),
                    p_y[2..].parse::<usize>().unwrap(),
                );

                let only_a = (p_x / a_x).min(p_y / a_y);

                for pushes_a in 0..=only_a {
                    let (target_x, target_y) = (p_x - pushes_a * a_x, p_y - pushes_a * a_y);

                    let pushes_b = (target_x / b_x).min(target_y / b_y);

                    if target_x == pushes_b * b_x && target_y == pushes_b * b_y {
                        debug_assert_eq!(target_x / b_x, target_y / b_y);
                        return pushes_a * 3 + pushes_b;
                    }
                }

                0
            })
            .sum()
    }
}

impl SolutionGold<usize, usize> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> usize {
        input
            .split("\n\n")
            .map(|conf| {
                let mut lines = conf.lines();
                let line_1 = lines.next().unwrap().split_once(": ").unwrap().1;
                let line_2 = lines.next().unwrap().split_once(": ").unwrap().1;
                let line_3 = lines.next().unwrap().split_once(": ").unwrap().1;

                let (a_x, a_y) = line_1.split_once(", ").unwrap();
                let (a_x, a_y) = (
                    a_x[2..].parse::<usize>().unwrap(),
                    a_y[2..].parse::<usize>().unwrap(),
                );
                let (b_x, b_y) = line_2.split_once(", ").unwrap();
                let (b_x, b_y) = (
                    b_x[2..].parse::<usize>().unwrap(),
                    b_y[2..].parse::<usize>().unwrap(),
                );
                let (p_x, p_y) = line_3.split_once(", ").unwrap();
                let (p_x, p_y) = (
                    p_x[2..].parse::<usize>().unwrap() + 10000000000000,
                    p_y[2..].parse::<usize>().unwrap() + 10000000000000,
                );

                let x = (b_y as f64 * p_x as f64 - b_x as f64 * p_y as f64)
                    / (a_x as f64 * b_y as f64 - b_x as f64 * a_y as f64);
                let y = (a_x as f64 * p_y as f64 - a_y as f64 * p_x as f64)
                    / (a_x as f64 * b_y as f64 - b_x as f64 * a_y as f64);

                if x.fract() > 0.0001 || y.fract() > 0.0001 {
                    0
                } else {
                    x as usize * 3 + y as usize
                }
            })
            .sum()
    }
}

/// https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

/// https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(480, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(37297, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(875318608908, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(83197086729371, output);
}
