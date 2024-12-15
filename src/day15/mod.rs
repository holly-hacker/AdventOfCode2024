use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 15;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let (grid, instructions) = input.split_once("\n\n").unwrap();
        let instructions = instructions.lines().collect::<String>();

        let mut grid = grid.as_bytes().to_vec();
        let width = grid.iter().position(|&c| c == b'\n').unwrap();
        let stride = width + 1;
        // let height = (grid.len() + 1) / stride;

        // apply instructions
        for instruction in instructions.chars() {
            let current_pos = grid.iter().position(|&c| c == b'@').unwrap();
            let current_pos_x = (current_pos % stride) as isize;
            let current_pos_y = (current_pos / stride) as isize;
            let direction = match instruction {
                '^' => (0isize, -1isize),
                'v' => (0, 1),
                '<' => (-1, 0),
                '>' => (1, 0),
                i => panic!("unknown instruction: '{i}'"),
            };
            let direction_i = direction.1 * stride as isize + direction.0;

            // see if movable
            let mut push_count = 0;
            let mut can_move = true;
            for i in 1.. {
                let current_pos_x_i = current_pos_x + direction.0 * i;
                let current_pos_y_i = current_pos_y + direction.1 * i;
                let current_pos_i = current_pos_y_i * stride as isize + current_pos_x_i;

                if grid[current_pos_i as usize] == b'#' {
                    can_move = false;
                    break;
                }
                if grid[current_pos_i as usize] == b'.' {
                    can_move = true;
                    break;
                }
                if grid[current_pos_i as usize] == b'O' {
                    push_count += 1;
                }
            }

            // println!("[{can_move}] Moving {:?} with push count {:?}",direction, push_count);

            // move if possible
            if can_move {
                // move the blocks over
                for i in (0..push_count).rev() {
                    grid[(current_pos as isize + direction_i * (i + 2)) as usize] =
                        grid[(current_pos as isize + direction_i * (i + 1)) as usize];
                }

                // move player
                grid[(current_pos as isize + direction_i) as usize] = b'@';
                grid[current_pos] = b'.';
            }
        }

        grid.into_iter()
            .enumerate()
            .filter(|(_, chr)| *chr == b'O')
            .map(|(i, _)| {
                let x = i % stride;
                let y = i / stride;
                y * 100 + x
            })
            .sum()
    }
}

impl SolutionGold<usize, usize> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> usize {
        let (grid, instructions) = input.split_once("\n\n").unwrap();
        let instructions = instructions.lines().collect::<String>();

        let grid = grid
            .replace("#", "##")
            .replace("O", "[]")
            .replace(".", "..")
            .replace("@", "@.");
        let mut grid = grid.as_bytes().to_vec();
        let width = grid.iter().position(|&c| c == b'\n').unwrap();
        let stride = width + 1;
        // let height = (grid.len() + 1) / stride;

        // apply instructions
        for instruction in instructions.chars() {
            let current_pos = grid.iter().position(|&c| c == b'@').unwrap();
            let current_pos_x = (current_pos % stride) as isize;
            let current_pos_y = (current_pos / stride) as isize;
            let direction = match instruction {
                '^' => (0isize, -1isize),
                'v' => (0, 1),
                '<' => (-1, 0),
                '>' => (1, 0),
                i => panic!("unknown instruction: '{i}'"),
            };
            let direction_i = direction.1 * stride as isize + direction.0;

            // see if movable
            let mut can_move = false;
            let mut pushes_box = false;
            {
                let current_pos_x_i = current_pos_x + direction.0;
                let current_pos_y_i = current_pos_y + direction.1;
                let current_pos_i = current_pos_y_i * stride as isize + current_pos_x_i;

                if grid[current_pos_i as usize] == b'#' {
                    can_move = false;
                }
                if grid[current_pos_i as usize] == b'.' {
                    can_move = true;
                }
                if matches!(grid[current_pos_i as usize], b'[' | b']') {
                    pushes_box = true;
                }
            }

            // move if possible
            if can_move
                || (pushes_box
                    && push_box_recursive(&mut grid, stride, current_pos, direction, false)
                    && push_box_recursive(&mut grid, stride, current_pos, direction, true))
            {
                // move player
                grid[(current_pos as isize + direction_i) as usize] = b'@';
                grid[current_pos] = b'.';
            }
        }

        grid.into_iter()
            .enumerate()
            .filter(|(_, chr)| *chr == b'[')
            .map(|(i, _)| {
                let x = i % stride;
                let y = i / stride;
                y * 100 + x
            })
            .sum()
    }
}

fn push_box_recursive(
    grid: &mut Vec<u8>,
    stride: usize,
    push_pos: usize,
    push_dir: (isize, isize),
    do_push: bool,
) -> bool {
    let next_pos = push_pos as isize + push_dir.1 * stride as isize + push_dir.0;

    let next_chr = grid[next_pos as usize];
    match grid[next_pos as usize] {
        b'#' => return false,
        b'.' => return true,
        b'[' => {
            // left
        }
        b']' => {
            // right
        }
        i => panic!("unknown char: '{i}'"),
    }

    let can_push = if push_dir.1 == 0 {
        // left/right
        let next_pos_2 = push_pos as isize + push_dir.1 * 2 * stride as isize + push_dir.0 * 2;
        push_box_recursive(grid, stride, next_pos_2 as usize, push_dir, do_push)
    } else {
        // top/down
        let other_push_pos = (next_pos
            + match next_chr {
                b'[' => 1isize,
                b']' => -1,
                _ => panic!("unknown char: '{next_chr}'"),
            }) as usize;

        let can_push_main = push_box_recursive(grid, stride, next_pos as usize, push_dir, do_push);
        let can_push_other = push_box_recursive(grid, stride, other_push_pos, push_dir, do_push);

        can_push_main && can_push_other
    };

    if can_push && do_push {
        if push_dir.1 == 0 {
            // left/right
            let next_pos_3 = push_pos as isize + push_dir.1 * 3 * stride as isize + push_dir.0 * 3;
            let next_pos_2 = push_pos as isize + push_dir.1 * 2 * stride as isize + push_dir.0 * 2;
            let next_pos_1 = push_pos as isize + push_dir.1 * 1 * stride as isize + push_dir.0 * 1;
            grid[next_pos_3 as usize] = grid[next_pos_2 as usize];
            grid[next_pos_2 as usize] = grid[next_pos_1 as usize];
            grid[next_pos_1 as usize] = b'.';
        } else {
            // top/down
            let other_box = match next_chr {
                b'[' => 1isize,
                b']' => -1,
                _ => panic!("unknown char: '{next_chr}'"),
            };

            let next_pos_2 = push_pos as isize + push_dir.1 * 2 * stride as isize + push_dir.0 * 2;
            let next_pos_1 = push_pos as isize + push_dir.1 * 1 * stride as isize + push_dir.0 * 1;

            grid[next_pos_2 as usize] = grid[next_pos_1 as usize];
            grid[(next_pos_2 + other_box) as usize] = grid[(next_pos_1 + other_box) as usize];

            grid[next_pos_1 as usize] = b'.';
            grid[(next_pos_1 + other_box) as usize] = b'.';

            // todo!("push top/down");
        }
    }

    can_push
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(2028, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(1414416, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(9021, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(1386070, output);
}
