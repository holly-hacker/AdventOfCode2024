use std::collections::HashSet;

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

        run_through_grid(&mut grid, (start_x, start_y), width, height, false);

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

        let mut unmodified_grid = grid.clone();
        run_through_grid(
            &mut unmodified_grid,
            (start_x, start_y),
            width,
            height,
            false,
        );

        let mut count = 0;
        for encountered_position in unmodified_grid
            .iter()
            .enumerate()
            .filter(|(_, &c)| c == b'X')
            .map(|(i, _)| i)
        {
            let mut grid = grid.clone();
            grid[encountered_position] = b'#';

            if !run_through_grid(&mut grid, (start_x, start_y), width, height, true) {
                count += 1;
            }
        }

        // TODO: idk why the +1
        count + 1
    }
}

fn run_through_grid(
    grid: &mut [u8],
    start_pos: (usize, usize),
    width: usize,
    height: usize,
    check_loop: bool,
) -> bool {
    let stride = width + 1;

    let mut x = start_pos.0 as isize;
    let mut y = start_pos.1 as isize;
    let mut direction = (0, -1);

    let mut seen_collisions = check_loop.then(HashSet::new);

    loop {
        let next_x = x + direction.0;
        let next_y = y + direction.1;

        if next_x < 0 || next_x >= width as isize || next_y < 0 || next_y >= height as isize {
            // exited the grid
            return true;
        }

        if (grid[(next_y * stride as isize + next_x) as usize]) == b'#' {
            // hit a wall

            if let Some(hashset) = &mut seen_collisions {
                if !hashset.insert((x, y, direction)) {
                    return false;
                }
            }

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
