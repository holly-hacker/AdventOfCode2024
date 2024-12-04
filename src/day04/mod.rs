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
                for (dir_x, dir_y) in [
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ] {
                    if dir_x < 0 && x < 3 {
                        continue;
                    }
                    if dir_x > 0 && x + 3 >= w {
                        continue;
                    }
                    if dir_y < 0 && y < 3 {
                        continue;
                    }
                    if dir_y > 0 && y + 3 >= h {
                        continue;
                    }

                    let y1 = (y as isize + dir_y as isize * 1) as usize;
                    let y2 = (y as isize + dir_y as isize * 2) as usize;
                    let y3 = (y as isize + dir_y as isize * 3) as usize;

                    let x1 = (x as isize + dir_x as isize * 1) as usize;
                    let x2 = (x as isize + dir_x as isize * 2) as usize;
                    let x3 = (x as isize + dir_x as isize * 3) as usize;

                    if grid[y][x] == b'X'
                        && grid[y1][x1] == b'M'
                        && grid[y2][x2] == b'A'
                        && grid[y3][x3] == b'S'
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
                // check center of the cross
                if grid[y][x] != b'A' {
                    continue;
                }

                // check diagonal top-left to bottom-right
                let check1 = (grid[y - 1][x - 1] == b'M' && grid[y + 1][x + 1] == b'S')
                    || (grid[y + 1][x + 1] == b'M' && grid[y - 1][x - 1] == b'S');

                // check diagonal top-right to bottom-left
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
