use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 25;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let mut keys = vec![];
        let mut locks = vec![];
        input.split("\n\n").for_each(|schematic| {
            let is_lock = schematic.starts_with("#####");

            let mut nums = [0u8; 5];
            for pin in 0..5 {
                let mut num = 0;
                for i in 0..5 {
                    if schematic.as_bytes()[6 * (i + 1) + pin] == b'#' {
                        num += 1;
                    }
                }
                nums[pin] = if is_lock { 5 - num } else { num };
            }

            if is_lock {
                locks.push(nums);
            } else {
                keys.push(nums);
            }
        });

        locks
            .into_iter()
            .map(|l| {
                keys.iter()
                    .filter(|k| {
                        k[0] <= l[0] && k[1] <= l[1] && k[2] <= l[2] && k[3] <= l[3] && k[4] <= l[4]
                    })
                    .count()
            })
            .sum()
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(3, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(2840, output);
}
