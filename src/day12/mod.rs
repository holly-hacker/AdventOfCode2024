use std::collections::HashSet;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 9;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let input = input.as_bytes();
        let width = input.iter().position(|&b| b == b'\n').unwrap();
        let stride = width + 1;
        let height = (input.len() + 1) / stride;

        let mut visited = vec![false; input.len()];
        let mut cum = 0;

        for y in 0..height {
            for x in 0..width {
                // todo
                if visited[y * stride + x] {
                    continue;
                }

                let chr = input[y * stride + x];

                let mut visited_now = HashSet::new();
                let mut perimeter_set = Vec::new();
                flood_fill(
                    chr,
                    input,
                    stride,
                    x,
                    y,
                    &mut visited_now,
                    &mut perimeter_set,
                );

                let area = visited_now.len();
                let perimeter = perimeter_set.len();

                visited_now.drain().for_each(|v| visited[v] = true);

                cum += area * perimeter;
            }
        }

        cum
    }
}

fn flood_fill(
    chr: u8,
    input: &[u8],
    stride: usize,
    x: usize,
    y: usize,
    visited: &mut HashSet<usize>,
    perimeter: &mut Vec<isize>,
) {
    if !visited.insert(y * stride + x) {
        return;
    }

    // check all 4 directions
    if x > 0 && input[y * stride + x - 1] == chr {
        flood_fill(chr, input, stride, x - 1, y, visited, perimeter);
    } else {
        perimeter.push(y as isize * stride as isize + x as isize - 1);
    }

    if x < stride - 2 && input[y * stride + x + 1] == chr {
        flood_fill(chr, input, stride, x + 1, y, visited, perimeter);
    } else {
        perimeter.push(y as isize * stride as isize + x as isize + 1);
    }

    if y > 0 && input[(y - 1) * stride + x] == chr {
        flood_fill(chr, input, stride, x, y - 1, visited, perimeter);
    } else {
        perimeter.push((y as isize - 1) * stride as isize + x as isize);
    }

    if y < (input.len() + 1) / stride - 1 && input[(y + 1) * stride + x] == chr {
        flood_fill(chr, input, stride, x, y + 1, visited, perimeter);
    } else {
        perimeter.push((y as isize + 1) * stride as isize + x as isize);
    }
}

impl SolutionGold<usize, usize> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> usize {
        let input = input.as_bytes();
        let width = input.iter().position(|&b| b == b'\n').unwrap();
        let stride = width + 1;
        let height = (input.len() + 1) / stride;

        let mut visited = vec![false; input.len()];
        let mut cum = 0;

        for y in 0..height {
            for x in 0..width {
                // todo
                if visited[y * stride + x] {
                    continue;
                }

                let chr = input[y * stride + x];

                let mut visited_now = HashSet::new();
                let mut perimeter_set = Vec::new();
                flood_fill_2(
                    chr,
                    input,
                    stride,
                    x,
                    y,
                    &mut visited_now,
                    &mut perimeter_set,
                );

                // calculate sides
                'check: loop {
                    for i in (0..perimeter_set.len() - 1).rev() {
                        for j in i + 1..perimeter_set.len() {
                            if i == j {
                                continue;
                            }
                            let dir_1 = perimeter_set[i].1;
                            let dir_2 = perimeter_set[j].1;

                            let start_1 = perimeter_set[i].0;
                            let start_2 = perimeter_set[j].0;

                            let len_1 = perimeter_set[i].2;
                            let len_2 = perimeter_set[j].2;

                            let end_excl_1 = (
                                start_1.0 + dir_1.0 * len_1 as isize,
                                start_1.1 + dir_1.1 * len_1 as isize,
                            );
                            let end_excl_2 = (
                                start_2.0 + dir_2.0 * len_2 as isize,
                                start_2.1 + dir_2.1 * len_2 as isize,
                            );

                            if dir_1 == dir_2 {
                                if end_excl_1 == start_2 {
                                    perimeter_set[i].2 += len_2;
                                    perimeter_set.remove(j);
                                    continue 'check;
                                } else if end_excl_2 == start_1 {
                                    perimeter_set[j].2 += len_1;
                                    perimeter_set.remove(i);
                                    continue 'check;
                                }
                            }
                        }
                    }

                    break;
                }

                let area = visited_now.len();
                let perimeter = perimeter_set.len();

                visited_now.drain().for_each(|v| visited[v] = true);

                cum += area * perimeter;
            }
        }

        cum
    }
}

fn flood_fill_2(
    chr: u8,
    input: &[u8],
    stride: usize,
    x: usize,
    y: usize,
    visited: &mut HashSet<usize>,
    perimeter: &mut Vec<((isize, isize), (isize, isize), usize)>,
) {
    if !visited.insert(y * stride + x) {
        return;
    }

    // check all 4 directions
    if x > 0 && input[y * stride + x - 1] == chr {
        flood_fill_2(chr, input, stride, x - 1, y, visited, perimeter);
    } else {
        perimeter.push(((x as isize - 1, y as isize), (0, -1), 1));
    }

    if x < stride - 2 && input[y * stride + x + 1] == chr {
        flood_fill_2(chr, input, stride, x + 1, y, visited, perimeter);
    } else {
        perimeter.push(((x as isize + 1, y as isize), (0, 1), 1));
    }

    if y > 0 && input[(y - 1) * stride + x] == chr {
        flood_fill_2(chr, input, stride, x, y - 1, visited, perimeter);
    } else {
        perimeter.push(((x as isize, y as isize - 1), (1, 0), 1));
    }

    if y < (input.len() + 1) / stride - 1 && input[(y + 1) * stride + x] == chr {
        flood_fill_2(chr, input, stride, x, y + 1, visited, perimeter);
    } else {
        perimeter.push(((x as isize, y as isize + 1), (-1, 0), 1));
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(1930, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(1424006, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(1206, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(858684, output);
}
