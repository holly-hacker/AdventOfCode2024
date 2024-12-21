use std::{collections::HashMap, iter};

use utils::fast_parse_int;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 21;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let mut cache = HashMap::new();
        input
            .lines()
            .map(|line| {
                let numeric_part = fast_parse_int(&line[..3]);

                let mut all_moves = 0;
                let mut current_char = 'A';
                for chr in line.chars() {
                    let new_moves = get_moves(current_char, chr, 0, 2, &mut cache);
                    all_moves += new_moves;
                    current_char = chr;
                }

                all_moves * numeric_part
            })
            .sum()
    }
}

impl SolutionGold<usize, usize> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> usize {
        let mut cache = HashMap::new();
        input
            .lines()
            .map(|line| {
                let numeric_part = fast_parse_int(&line[..3]);

                let mut all_moves = 0;
                let mut current_char = 'A';
                for chr in line.chars() {
                    let new_moves = get_moves(current_char, chr, 0, 25, &mut cache);
                    all_moves += new_moves;
                    current_char = chr;
                }

                all_moves * numeric_part
            })
            .sum()
    }
}

fn get_moves(
    start_char: char,
    target: char,
    depth: usize,
    max_depth: usize,
    memoization: &mut HashMap<((isize, isize, bool), usize), usize>,
) -> usize {
    let start_pos = get_position(start_char, depth);
    let end_pos = get_position(target, depth);

    let diff_x = end_pos.0 as isize - start_pos.0 as isize;
    let diff_y = end_pos.1 as isize - start_pos.1 as isize;

    // determine which direction goes first to avoid hovering over empty space
    let y_must_first = if depth == 0 {
        start_pos.1 == 3 && end_pos.0 == 0
    } else {
        start_pos.1 == 0 && end_pos.0 == 0
    };
    let x_must_first = if depth == 0 {
        end_pos.1 == 3 && start_pos.0 == 0
    } else {
        end_pos.1 == 0 && start_pos.0 == 0
    };

    let x_first = match (x_must_first, y_must_first) {
        (true, false) => true,
        (false, true) => false,
        (false, false) => match (diff_x.signum(), diff_y.signum()) {
            // moving to the left is expensive, prefer doing those first so they get done in one go
            // in order of preference: left, down, up, right
            (-1, _) => true,  // left first
            (_, 1) => false,  // then down
            (_, -1) => false, // then up
            // (1, _) => false,  // then right (though it doesn't matter)
            // (0, 0) => true,   // doesn't matter
            // d => unreachable!("Invalid diff: {d:?}"),
            _ => false,
        },
        // (false, false) => &[false],
        (true, true) => unreachable!("Both x and y must go first"),
    };

    if let Some(&moves) = memoization.get(&((diff_x, diff_y, x_first), depth)) {
        return moves;
    }

    let x_moves = iter::repeat_n(if diff_x < 0 { '<' } else { '>' }, diff_x.unsigned_abs())
        .collect::<String>();
    let y_moves = iter::repeat_n(if diff_y < 0 { '^' } else { 'v' }, diff_y.unsigned_abs())
        .collect::<String>();
    let all_moves = if x_first {
        x_moves + &y_moves + "A"
    } else {
        y_moves + &x_moves + "A"
    };

    if depth == max_depth {
        return all_moves.len();
    }

    let mut total_len = 0;
    let mut current_char = 'A';
    for chr in all_moves.chars() {
        let move_len = get_moves(current_char, chr, depth + 1, max_depth, memoization);
        total_len += move_len;

        current_char = chr;
    }

    memoization.insert(((diff_x, diff_y, x_first), depth), total_len);

    total_len
}

fn get_position(input: char, depth: usize) -> (usize, usize) {
    if depth == 0 {
        match input {
            '7' => (0, 0),
            '8' => (1, 0),
            '9' => (2, 0),
            '4' => (0, 1),
            '5' => (1, 1),
            '6' => (2, 1),
            '1' => (0, 2),
            '2' => (1, 2),
            '3' => (2, 2),
            '0' => (1, 3),
            'A' => (2, 3),
            chr => unreachable!("Invalid char: {chr}"),
        }
    } else {
        match input {
            '^' => (1, 0),
            'A' => (2, 0),
            '<' => (0, 1),
            'v' => (1, 1),
            '>' => (2, 1),
            chr => unreachable!("Invalid char: {chr}"),
        }
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(126384, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    // not 258330
    assert_eq!(246990, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(154115708116294, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(306335137543664, output);
}
