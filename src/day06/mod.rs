use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 6;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let mut grid = input.as_bytes().to_vec();
        let width = input.find('\n').unwrap();
        let stride = width + 1;
        let height = (input.len() + 1) / stride;
        let start_pos = input.find('^').unwrap();
        let start_x = start_pos % stride;
        let start_y = start_pos / stride;

        let mut x = start_x as isize;
        let mut y = start_y as isize;

        let mut direction = (0, -1);
        loop {
            let next_x = x + direction.0;
            let next_y = y + direction.1;

            if next_x < 0 || next_x >= width as isize || next_y < 0 || next_y >= height as isize {
                // exited the grid
                break;
            }

            if (grid[(next_y * stride as isize + next_x) as usize]) == b'#' {
                // hit a wall
                direction = match direction {
                    (0, -1) => (1, 0),
                    (1, 0) => (0, 1),
                    (0, 1) => (-1, 0),
                    (-1, 0) => (0, -1),
                    _ => unreachable!(),
                };
                continue;
            }

            grid[(y * stride as isize + x) as usize] = b'X';

            x = next_x;
            y = next_y;
        }

        grid.iter().filter(|&&c| c == b'X').count() + 1
    }
}

impl SolutionGold<usize, usize> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> usize {
        let grid = input.as_bytes().to_vec();
        let width = input.find('\n').unwrap();
        let stride = width + 1;
        let height = (input.len() + 1) / stride;
        let start_pos = input.find('^').unwrap();
        let start_x = start_pos % stride;
        let start_y = start_pos / stride;

        let mut count = 0;
        for insert_x in 0..width {
            for insert_y in 0..height {
                let mut grid = grid.clone();
                grid[insert_y * stride + insert_x] = b'#';

                let mut x = start_x as isize;
                let mut y = start_y as isize;

                let mut direction = (0, -1);
                let mut iterations = 0;
                while iterations < 10_000 {
                    let next_x = x + direction.0;
                    let next_y = y + direction.1;

                    if next_x < 0
                        || next_x >= width as isize
                        || next_y < 0
                        || next_y >= height as isize
                    {
                        // exited the grid
                        break;
                    }

                    if (grid[(next_y * stride as isize + next_x) as usize]) == b'#' {
                        // hit a wall
                        direction = match direction {
                            (0, -1) => (1, 0),
                            (1, 0) => (0, 1),
                            (0, 1) => (-1, 0),
                            (-1, 0) => (0, -1),
                            _ => unreachable!(),
                        };
                        continue;
                    }

                    grid[(y * stride as isize + x) as usize] = b'X';

                    x = next_x;
                    y = next_y;
                    iterations += 1;
                }

                if iterations == 10_000 {
                    count += 1;
                }
            }
        }

        count
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(41, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(4656, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(6, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(1575, output);
}
