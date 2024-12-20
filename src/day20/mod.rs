use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 20;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let grid = input.as_bytes();
        let width = grid.iter().position(|&c| c == b'\n').unwrap();
        let stride = width + 1;
        let height = (grid.len() + 1) / stride;

        let start_pos = grid.iter().position(|&c| c == b'S').unwrap();
        let end_pos = grid.iter().position(|&c| c == b'E').unwrap();

        // walk through the track and get all positions
        let mut path = Vec::new();
        path.push(start_pos);
        let mut current_pos = start_pos;
        let mut prev_pos = start_pos;
        loop {
            if current_pos == end_pos {
                break;
            }

            for offset in [1isize, -1, stride as isize, -(stride as isize)] {
                let next_pos = (current_pos as isize + offset) as usize;
                if next_pos != prev_pos && grid[next_pos] != b'#' {
                    prev_pos = current_pos;
                    current_pos = next_pos;
                    path.push(current_pos);
                    break;
                }
            }
        }

        // dbg!(path.len());

        // loop over possible cheat positions
        let mut good_count = 0;
        for i in 0..path.len() {
            let pos = path[i];
            let pos_x = pos % stride;
            let pos_y = pos / stride;

            for offset in [1isize, -1, stride as isize, -(stride as isize)] {
                if offset == -1 && pos_x <= 1 {
                    continue;
                }
                if offset == 1 && pos_x >= width - 2 {
                    continue;
                }
                if offset == -(stride as isize) && pos_y <= 1 {
                    continue;
                }
                if offset == stride as isize && pos_y >= height - 2 {
                    continue;
                }

                let next_pos_1 = (pos as isize + offset) as usize;
                let next_pos_2 = (pos as isize + offset * 2) as usize;

                if grid[next_pos_1] == b'#' && grid[next_pos_2] != b'#' {
                    let next_i = path.iter().position(|&p| p == next_pos_2).unwrap();

                    if next_i > i {
                        let skipped = (next_i - i) - 2;
                        // println!("Skipping from {i} ({pos}) to {next_i} ({next_pos_2}), skipping {skipped}");

                        if skipped >= 100 {
                            good_count += 1;
                        }
                    }
                }
            }
        }

        good_count
    }
}

impl SolutionGold<usize, usize> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> usize {
        let grid = input.as_bytes();
        let width = grid.iter().position(|&c| c == b'\n').unwrap();
        let stride = width + 1;
        let height = (grid.len() + 1) / stride;

        let start_pos = grid.iter().position(|&c| c == b'S').unwrap();
        let end_pos = grid.iter().position(|&c| c == b'E').unwrap();
        let end_pos_x = (end_pos % stride) as isize;
        let end_pos_y = (end_pos / stride) as isize;

        // walk through the track and get all positions
        let mut path = Vec::new();
        path.push(start_pos);
        let mut current_pos = start_pos;
        let mut prev_pos = start_pos;
        loop {
            if current_pos == end_pos {
                break;
            }

            for offset in [1isize, -1, stride as isize, -(stride as isize)] {
                let next_pos = (current_pos as isize + offset) as usize;
                if next_pos != prev_pos && grid[next_pos] != b'#' {
                    prev_pos = current_pos;
                    current_pos = next_pos;
                    path.push(current_pos);
                    break;
                }
            }
        }

        // dbg!(path.len());

        // loop over possible cheat positions
        let mut good_count = 0;
        for i in 0..path.len() {
            let pos = path[i];
            let pos_x = (pos % stride) as isize;
            let pos_y = (pos / stride) as isize;

            // find all grid positions with a manhattan distance of 6 or less

            for offs_y in -20isize..=20 {
                let new_y = pos_y + offs_y;
                if new_y < 0 || new_y >= height as isize {
                    continue;
                }

                for offs_x in -20isize..=20 {
                    let new_x = pos_x + offs_x;
                    if new_x < 0 || new_x >= width as isize {
                        continue;
                    }

                    // a bit subobtimal, but whatever
                    let manhattan_dist = (offs_x.abs() + offs_y.abs()) as usize;
                    if manhattan_dist > 20 || manhattan_dist <= 1 {
                        continue;
                    }

                    let new_pos = (new_x + new_y * stride as isize) as usize;

                    // don't teleport into a wall
                    if grid[new_pos] == b'#' {
                        continue;
                    }

                    // IDEA: don't count cheats that walk over the finish
                    // seems to be incorrect in the problem statement?
                    if false {
                        let dist_end_x = end_pos_x - pos_x;
                        let dist_end_y = end_pos_y - pos_y;
                        if dist_end_x == 0 {
                            // same y value
                            if dist_end_y.signum() == offs_y.signum()
                                && dist_end_y.unsigned_abs() < manhattan_dist
                            {
                                continue;
                            }
                        }

                        if dist_end_y == 0 {
                            // same x value
                            if dist_end_x.signum() == offs_x.signum()
                                && dist_end_x.unsigned_abs() < manhattan_dist
                            {
                                continue;
                            }
                        }
                    }

                    let next_i = path.iter().position(|&p| p == new_pos).unwrap();
                    if next_i <= i || next_i - i <= manhattan_dist {
                        continue;
                    }
                    let skipped = (next_i - i) - manhattan_dist;

                    debug_assert!(skipped % 2 == 0);

                    if skipped >= 100 {
                        good_count += 1;
                    }
                }
            }
        }

        // todo!();
        good_count
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(0, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(1381, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(16, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    // not 35300
    // not 968652
    // not 977937 (checked twice)
    assert_eq!(712058625427487, output);
}
