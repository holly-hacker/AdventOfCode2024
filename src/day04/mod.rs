use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 4;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let input = input.as_bytes();
        let w = input.iter().position(|&c| c == b'\n').unwrap();
        let stride = w + 1;
        let h = (input.len() + 1) / stride;

        // left and right are fastest to check since those bytes are consecutive
        // we can use standard search algorithms for that which utilize SIMD
        let count_ltr = memchr::memmem::find_iter(input, b"XMAS").count();
        let count_rtl = memchr::memmem::find_iter(input, b"SAMX").count();

        // read the other directions normally
        let mut count = 0;
        for y in 0..h {
            for x in 0..w {
                let cur_pos = input[y * stride + x];
                if cur_pos != b'X' && cur_pos != b'S' {
                    continue;
                }

                for (dir_x, dir_y) in [(0, 1), (1, -1), (1, 1)] {
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

                    let read = [
                        cur_pos,
                        input[y1 * stride + x1],
                        input[y2 * stride + x2],
                        input[y3 * stride + x3],
                    ];

                    // check for a match in both directions
                    if &read == b"XMAS" || &read == b"SAMX" {
                        count += 1;
                    }
                }
            }
        }

        count_ltr + count_rtl + count
    }
}

impl SolutionGold<usize, usize> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> usize {
        let input = input.as_bytes();
        let w = input.iter().position(|&c| c == b'\n').unwrap();
        let stride = w + 1;
        let h = (input.len() + 1) / stride;

        let mut count = 0;
        for y in 1..h - 1 {
            for x in 1..w - 1 {
                // check center of the cross
                if input[y * stride + x] != b'A' {
                    continue;
                }

                // check diagonals
                let top_left = input[(y - 1) * stride + x - 1];
                let top_right = input[(y - 1) * stride + x + 1];
                let bottom_left = input[(y + 1) * stride + x - 1];
                let bottom_right = input[(y + 1) * stride + x + 1];

                let check1 = (top_left == b'M' && bottom_right == b'S')
                    || (bottom_right == b'M' && top_left == b'S');
                let check2 = (bottom_left == b'M' && top_right == b'S')
                    || (top_right == b'M' && bottom_left == b'S');

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
