use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 9;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let input = input.as_bytes();

        let mut disk = vec![];
        let mut is_file = true;
        let mut last_file_id = 0usize;
        for file in input {
            let file_len = (file - b'0') as usize;
            if is_file {
                for _ in 0..file_len {
                    disk.push(Some(last_file_id));
                }
                last_file_id += 1;
            } else {
                for _ in 0..file_len {
                    disk.push(None);
                }
            }

            is_file = !is_file;
        }

        // compact
        'outer: for i in (1..disk.len()).rev() {
            if let Some(file_id) = disk[i] {
                for j in 0..i {
                    if disk[j].is_none() {
                        disk[j] = Some(file_id);
                        disk[i] = None;
                        continue 'outer;
                    }
                }
            }
        }

        disk.into_iter()
            .enumerate()
            .fold(0, |acc, (i, x)| acc + i * x.unwrap_or_default())
    }
}

impl SolutionGold<usize, usize> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> usize {
        let input = input.as_bytes();

        let mut disk = vec![];
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
        for i in (1..disk.len()).rev() {
            if let (Some(file_id), file_len) = disk[i] {
                for j in 0..i {
                    if let (None, empty_len) = disk[j] {
                        if empty_len >= file_len {
                            disk[i] = (None, file_len);

                            disk.insert(j, (Some(file_id), file_len));
                            disk[j + 1].1 -= file_len;
                            break;
                        }
                    }
                }
            }
        }

        // expand
        let mut expanded_disk = vec![];
        for (file_id, file_len) in disk {
            if let Some(file_id) = file_id {
                for _ in 0..file_len {
                    expanded_disk.push(Some(file_id));
                }
            } else {
                for _ in 0..file_len {
                    expanded_disk.push(None);
                }
            }
        }

        expanded_disk
            .into_iter()
            .enumerate()
            .fold(0, |acc, (i, x)| acc + i * x.unwrap_or_default())
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
