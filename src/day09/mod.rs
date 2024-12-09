use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 9;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let input = input.as_bytes();

        let mut disk = Vec::with_capacity(input.len());
        let mut is_file = true;
        let mut last_file_id = 0usize;
        for file in input {
            let file_len = (file - b'0') as usize;
            if is_file {
                disk.push((Some(last_file_id), file_len));
                last_file_id += 1;
            } else {
                disk.push((None, file_len));
            }

            is_file = !is_file;
        }

        // compact
        let mut i_src = disk.len() - 1;
        'outer: loop {
            if disk[i_src].0.is_none() {
                i_src -= 1;
                continue;
            }

            let mut i_dst = 0;
            loop {
                // skip file blocks and empty blocks
                if disk[i_dst].0.is_some() || disk[i_dst].1 == 0 {
                    i_dst += 1;
                    continue;
                }

                // move what we can
                let to_move = disk[i_dst].1.min(disk[i_src].1);
                let left_over = disk[i_src].1 - to_move;
                let val = disk[i_src].0;
                if to_move == 0 {
                    break 'outer; // weird edge condition for the end of the compacted disk
                }
                debug_assert_ne!(to_move, 0);
                debug_assert!(val.is_some());

                disk[i_src].1 -= to_move;
                disk[i_dst].1 -= to_move;
                debug_assert_eq!(disk[i_src].1, left_over);

                // insert before dest
                disk.insert(i_dst, (val, to_move)); // TODO: slow bc moves everything after it
                i_src += 1;

                if disk[i_src].1 == 0 {
                    disk[i_src].0 = None;
                    i_src -= 1;
                    break;
                }

                i_dst += 1;
                if i_dst >= i_src {
                    break;
                }
            }

            i_src -= 1;
            if i_src == 0 {
                break;
            }
        }

        disk.into_iter()
            .fold((0, 0), |(acc_idx, acc_checksum), (file_val, file_len)| {
                if let Some(file_val) = file_val {
                    (
                        acc_idx + file_len,
                        acc_checksum
                            + match file_len {
                                0 => 0,
                                _ => {
                                    let range_center = acc_idx as f32 + (file_len - 1) as f32 * 0.5;
                                    let range_indices_sum =
                                        (range_center * file_len as f32) as usize;
                                    range_indices_sum * file_val
                                }
                            },
                    )
                } else {
                    (acc_idx + file_len, acc_checksum)
                }
            })
            .1
    }
}

impl SolutionGold<usize, usize> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> usize {
        let input = input.as_bytes();

        let mut disk = Vec::with_capacity(input.len());
        let mut is_file = true;
        let mut last_file_id = 0usize;
        for file in input {
            let file_len = (file - b'0') as usize;
            if is_file {
                disk.push((Some(last_file_id), file_len));
                last_file_id += 1;
            } else {
                disk.push((None, file_len));
            }

            is_file = !is_file;
        }

        // compact
        for i in (0..disk.len()).rev() {
            if let (Some(file_id), file_len) = disk[i] {
                for j in 0..i {
                    if let (None, empty_len) = disk[j] {
                        if empty_len >= file_len {
                            disk[i] = (None, file_len);

                            disk.insert(j, (Some(file_id), file_len)); // TODO: slow bc moves everything after it
                            disk[j + 1].1 -= file_len;
                            break;
                        }
                    }
                }
            }
        }

        disk.into_iter()
            .fold((0, 0), |(acc_idx, acc_checksum), (file_val, file_len)| {
                if let Some(file_val) = file_val {
                    (
                        acc_idx + file_len,
                        acc_checksum
                            + match file_len {
                                0 => 0,
                                _ => {
                                    let range_center = acc_idx as f32 + (file_len - 1) as f32 * 0.5;
                                    let range_indices_sum =
                                        (range_center * file_len as f32) as usize;
                                    range_indices_sum * file_val
                                }
                            },
                    )
                } else {
                    (acc_idx + file_len, acc_checksum)
                }
            })
            .1
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(1928, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(6384282079460, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(2858, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(6408966547049, output);
}
