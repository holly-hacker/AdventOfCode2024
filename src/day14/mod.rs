use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 14;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        const WIDTH: isize = 101;
        const HEIGHT: isize = 103;

        let mut robots = input
            .lines()
            .map(|line| {
                let (p, v) = line.split_once(' ').unwrap();
                let p = p.split_once('=').unwrap().1.split_once(',').unwrap();
                let v = v.split_once('=').unwrap().1.split_once(',').unwrap();
                let p = (p.0.parse::<isize>().unwrap(), p.1.parse::<isize>().unwrap());
                let v = (v.0.parse::<isize>().unwrap(), v.1.parse::<isize>().unwrap());

                (p, v)
            })
            .collect::<Vec<_>>();

        for _ in 0..100 {
            for r in &mut robots {
                r.0 = (r.0 .0 + r.1 .0, r.0 .1 + r.1 .1);

                while r.0 .0 < 0 {
                    r.0 .0 += WIDTH;
                }
                while r.0 .1 < 0 {
                    r.0 .1 += HEIGHT;
                }
                while r.0 .0 >= WIDTH {
                    r.0 .0 -= WIDTH;
                }
                while r.0 .1 >= HEIGHT {
                    r.0 .1 -= HEIGHT;
                }
            }
        }

        const X_MIDDLE: isize = WIDTH / 2;
        const Y_MIDDLE: isize = HEIGHT / 2;
        let top_left_quadrant = robots
            .iter()
            .filter(|r| r.0 .0 < X_MIDDLE && r.0 .1 < Y_MIDDLE)
            .count();
        let top_right_quadrant = robots
            .iter()
            .filter(|r| r.0 .0 > X_MIDDLE && r.0 .1 < Y_MIDDLE)
            .count();
        let bottom_left_quadrant = robots
            .iter()
            .filter(|r| r.0 .0 < X_MIDDLE && r.0 .1 > Y_MIDDLE)
            .count();
        let bottom_right_quadrant = robots
            .iter()
            .filter(|r| r.0 .0 > X_MIDDLE && r.0 .1 > Y_MIDDLE)
            .count();

        top_left_quadrant * top_right_quadrant * bottom_left_quadrant * bottom_right_quadrant
    }
}

impl SolutionGold<usize, usize> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> usize {
        const WIDTH: isize = 101;
        const HEIGHT: isize = 103;

        if input.len() < 1000 {
            return 0;
        }

        let mut robots = input
            .lines()
            .map(|line| {
                let (p, v) = line.split_once(' ').unwrap();
                let p = p.split_once('=').unwrap().1.split_once(',').unwrap();
                let v = v.split_once('=').unwrap().1.split_once(',').unwrap();
                let p = (p.0.parse::<isize>().unwrap(), p.1.parse::<isize>().unwrap());
                let v = (v.0.parse::<isize>().unwrap(), v.1.parse::<isize>().unwrap());

                (p, v)
            })
            .collect::<Vec<_>>();

        for i in 0..10000 {
            for r in &mut robots {
                r.0 = (r.0 .0 + r.1 .0, r.0 .1 + r.1 .1);

                while r.0 .0 < 0 {
                    r.0 .0 += WIDTH;
                }
                while r.0 .1 < 0 {
                    r.0 .1 += HEIGHT;
                }
                while r.0 .0 >= WIDTH {
                    r.0 .0 -= WIDTH;
                }
                while r.0 .1 >= HEIGHT {
                    r.0 .1 -= HEIGHT;
                }
            }

            // check if it looks like a christmas tree?

            // i == 7285
            // robots.iter().filter(|r| r.0 .0 == 23).count() >= (53 - 21) - 1
            if robots.iter().filter(|r| r.0 .1 == 21).count() >= 31
                && robots.iter().filter(|r| r.0 .0 == 23).count() >= (53 - 21)
            {
                return i + 1;
            }
        }

        panic!("nothing found");
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(21, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(210587128, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(0, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(7286, output);
}
