use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 4;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let h = input.lines().count();
        let w = input.lines().next().unwrap().trim_end().len();
        let grid: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();

        let mut count = 0;
        for y in 0..h {
            for x in 0..w {
                // check down
                if y + 3 < h {
                    if grid[y][x] == b'X'
                        && grid[y + 1][x] == b'M'
                        && grid[y + 2][x] == b'A'
                        && grid[y + 3][x] == b'S'
                    {
                        count += 1;
                    }
                }

                // check up
                if y >= 3 {
                    if grid[y][x] == b'X'
                        && grid[y - 1][x] == b'M'
                        && grid[y - 2][x] == b'A'
                        && grid[y - 3][x] == b'S'
                    {
                        count += 1;
                    }
                }

                // check left
                if x + 3 < w {
                    if grid[y][x] == b'X'
                        && grid[y][x + 1] == b'M'
                        && grid[y][x + 2] == b'A'
                        && grid[y][x + 3] == b'S'
                    {
                        count += 1;
                    }
                }

                // check up
                if x >= 3 {
                    if grid[y][x] == b'X'
                        && grid[y][x - 1] == b'M'
                        && grid[y][x - 2] == b'A'
                        && grid[y][x - 3] == b'S'
                    {
                        count += 1;
                    }
                }

                // check down-right
                if y + 3 < h && x + 3 < w {
                    if grid[y][x] == b'X'
                        && grid[y + 1][x + 1] == b'M'
                        && grid[y + 2][x + 2] == b'A'
                        && grid[y + 3][x + 3] == b'S'
                    {
                        count += 1;
                    }
                }

                // check up-left
                if y >= 3 && x >= 3 {
                    if grid[y][x] == b'X'
                        && grid[y - 1][x - 1] == b'M'
                        && grid[y - 2][x - 2] == b'A'
                        && grid[y - 3][x - 3] == b'S'
                    {
                        count += 1;
                    }
                }

                // check down-left
                if y + 3 < h && x >= 3 {
                    if grid[y][x] == b'X'
                        && grid[y + 1][x - 1] == b'M'
                        && grid[y + 2][x - 2] == b'A'
                        && grid[y + 3][x - 3] == b'S'
                    {
                        count += 1;
                    }
                }

                // check up-right
                if y >= 3 && x + 3 < w {
                    if grid[y][x] == b'X'
                        && grid[y - 1][x + 1] == b'M'
                        && grid[y - 2][x + 2] == b'A'
                        && grid[y - 3][x + 3] == b'S'
                    {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

impl SolutionGold<usize, usize> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> usize {
        let h = input.lines().count();
        let w = input.lines().next().unwrap().trim_end().len();
        let grid: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();

        let mut count = 0;
        for y in 1..h - 1 {
            for x in 1..w - 1 {
                if grid[y][x] != b'A' {
                    continue;
                }

                let check1 = (grid[y - 1][x - 1] == b'M' && grid[y + 1][x + 1] == b'S')
                    || (grid[y + 1][x + 1] == b'M' && grid[y - 1][x - 1] == b'S');

                let check2 = (grid[y + 1][x - 1] == b'M' && grid[y - 1][x + 1] == b'S')
                    || (grid[y - 1][x + 1] == b'M' && grid[y + 1][x - 1] == b'S');

                if check1 && check2 {
                    count += 1
                }
            }
        }

        count
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(18, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(2575, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(9, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(2041, output);
}
