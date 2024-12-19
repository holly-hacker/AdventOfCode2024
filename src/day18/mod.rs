use std::collections::{HashMap, HashSet};

use utils::fast_parse_int;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 18;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let nums = input
            .lines()
            .map(|line| line.split_once(',').unwrap())
            .map(|(a, b)| (fast_parse_int(a), fast_parse_int(b)))
            .collect::<Vec<_>>();

        let start_pos = (0, 0);
        let end_pos = if nums.len() > 100 { (70, 70) } else { (6, 6) };

        let mut pq = std::collections::BinaryHeap::<Node>::new();
        pq.push(Node {
            pos: start_pos,
            dist: 0,
        });

        let blocks = nums
            .iter()
            .take(if nums.len() > 100 { 1024 } else { 12 })
            .map(|(x, y)| (*x as u8, *y as u8))
            .collect::<HashSet<(u8, u8)>>();

        let mut visited = HashMap::<(u8, u8), usize, _>::new();

        while let Some(next) = pq.pop() {
            if next.pos == end_pos {
                return next.dist;
            }

            let (x, y) = next.pos;
            let dist = next.dist + 1;

            if x < end_pos.0 && !blocks.contains(&(x + 1, y)) {
                let entry = visited.entry((x + 1, y)).or_insert(usize::MAX);
                if *entry > dist {
                    *entry = dist;
                    pq.push(Node {
                        pos: (x + 1, y),
                        dist,
                    });
                }
            }
            if y < end_pos.1 && !blocks.contains(&(x, y + 1)) {
                let entry = visited.entry((x, y + 1)).or_insert(usize::MAX);
                if *entry > dist {
                    *entry = dist;
                    pq.push(Node {
                        pos: (x, y + 1),
                        dist,
                    });
                }
            }
            if x > 0 && !blocks.contains(&(x - 1, y)) {
                let entry = visited.entry((x - 1, y)).or_insert(usize::MAX);
                if *entry > dist {
                    *entry = dist;
                    pq.push(Node {
                        pos: (x - 1, y),
                        dist,
                    });
                }
            }
            if y > 0 && !blocks.contains(&(x, y - 1)) {
                let entry = visited.entry((x, y - 1)).or_insert(usize::MAX);
                if *entry > dist {
                    *entry = dist;
                    pq.push(Node {
                        pos: (x, y - 1),
                        dist,
                    });
                }
            }
        }

        unreachable!();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    pos: (u8, u8),
    dist: usize,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let dist_from_start = (self.pos.0 + self.pos.1)
            .cmp(&(other.pos.0 + other.pos.1))
            // .reverse()
            ;
        self.dist.cmp(&other.dist).reverse().then(dist_from_start)
    }
}

impl SolutionGold<usize, String> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> String {
        let nums = input
            .lines()
            .map(|line| line.split_once(',').unwrap())
            .map(|(a, b)| (fast_parse_int(a), fast_parse_int(b)))
            .collect::<Vec<_>>();

        let start_pos = (0, 0);
        let end_pos = if nums.len() > 100 { (70, 70) } else { (6, 6) };
        let start_i = if nums.len() > 100 { 1024 } else { 12 };

        for i in start_i..nums.len() {
            let mut pq = std::collections::BinaryHeap::<Node>::new();
            pq.push(Node {
                pos: start_pos,
                dist: 0,
            });

            let blocks = nums
                .iter()
                .take(i)
                .map(|(x, y)| (*x as u8, *y as u8))
                .collect::<HashSet<(u8, u8)>>();

            let mut visited = HashMap::<(u8, u8), usize, _>::new();

            let mut found_exit = false;
            while let Some(next) = pq.pop() {
                if next.pos == end_pos {
                    found_exit = true;
                    break;
                }

                let (x, y) = next.pos;
                let dist = next.dist + 1;

                if x < end_pos.0 && !blocks.contains(&(x + 1, y)) {
                    let entry = visited.entry((x + 1, y)).or_insert(usize::MAX);
                    if *entry > dist {
                        *entry = dist;
                        pq.push(Node {
                            pos: (x + 1, y),
                            dist,
                        });
                    }
                }
                if y < end_pos.1 && !blocks.contains(&(x, y + 1)) {
                    let entry = visited.entry((x, y + 1)).or_insert(usize::MAX);
                    if *entry > dist {
                        *entry = dist;
                        pq.push(Node {
                            pos: (x, y + 1),
                            dist,
                        });
                    }
                }
                if x > 0 && !blocks.contains(&(x - 1, y)) {
                    let entry = visited.entry((x - 1, y)).or_insert(usize::MAX);
                    if *entry > dist {
                        *entry = dist;
                        pq.push(Node {
                            pos: (x - 1, y),
                            dist,
                        });
                    }
                }
                if y > 0 && !blocks.contains(&(x, y - 1)) {
                    let entry = visited.entry((x, y - 1)).or_insert(usize::MAX);
                    if *entry > dist {
                        *entry = dist;
                        pq.push(Node {
                            pos: (x, y - 1),
                            dist,
                        });
                    }
                }
            }

            if !found_exit {
                return format!("{},{}", nums[i - 1].0, nums[i - 1].1);
            }
        }

        unreachable!("no solution found")
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(22, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(344, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!("6,1", output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!("46,18", output);
}
