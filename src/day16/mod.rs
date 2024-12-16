use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    rc::Rc,
    usize,
};

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 16;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let grid = input.as_bytes();
        let width = grid.iter().position(|&c| c == b'\n').unwrap();
        let stride = width + 1;

        let start_pos = grid.iter().position(|&c| c == b'S').unwrap();
        let end_pos = grid.iter().position(|&c| c == b'E').unwrap();

        let start_dir = (1isize, 0);

        let mut pq = BinaryHeap::<MazeEntrySilver>::new();

        pq.push(MazeEntrySilver {
            pos: start_pos,
            dir: start_dir,
            score: 0,
        });

        let mut history = HashSet::new();
        history.insert((start_pos, start_dir));

        while let Some(cur) = pq.pop() {
            // dbg!(&cur);
            // check if valid
            if cur.pos == end_pos {
                return cur.score;
            }

            let cur_x = (cur.pos % stride) as isize;
            let cur_y = (cur.pos / stride) as isize;

            // go forward, if possible
            let next_pos = (cur_y + cur.dir.1) * stride as isize + (cur_x + cur.dir.0);
            if grid[next_pos as usize] != b'#' && history.insert((next_pos as usize, cur.dir)) {
                pq.push(MazeEntrySilver {
                    pos: next_pos as usize,
                    dir: cur.dir,
                    score: cur.score + 1,
                });
            }

            // try turning left and right
            let new_dirs = match cur.dir {
                (0, 1) => [(1, 0), (-1, 0)],
                (0, -1) => [(-1, 0), (1, 0)],
                (1, 0) => [(0, -1), (0, 1)],
                (-1, 0) => [(0, 1), (0, -1)],
                _ => unreachable!("invalid dir: {:?}", cur.dir),
            };
            for new_dir in new_dirs {
                if history.insert((cur.pos, new_dir)) {
                    pq.push(MazeEntrySilver {
                        pos: cur.pos,
                        dir: new_dir,
                        score: cur.score + 1000,
                    });
                }
            }
        }

        unreachable!()
    }
}

#[derive(Debug, Eq, PartialEq)]
struct MazeEntrySilver {
    pos: usize,
    dir: (isize, isize),
    score: usize,
}

impl PartialOrd for MazeEntrySilver {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MazeEntrySilver {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score).reverse()
    }
}

impl SolutionGold<usize, usize> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> usize {
        let grid = input.as_bytes();
        let width = grid.iter().position(|&c| c == b'\n').unwrap();
        let stride = width + 1;

        let start_pos = grid.iter().position(|&c| c == b'S').unwrap();
        let end_pos = grid.iter().position(|&c| c == b'E').unwrap();

        let start_x = start_pos % stride;
        let start_y = start_pos / stride;
        let end_x = end_pos % stride;
        let end_y = end_pos / stride;

        let start_dir = (1isize, 0);

        let mut pq = BinaryHeap::<MazeEntryGold>::new();

        pq.push(MazeEntryGold {
            pos: (start_x, start_y),
            dir: start_dir,
            cost: 0,
            distance_to_end: end_x.abs_diff(start_x) + end_y.abs_diff(start_y),
            visited: LinkedListNode::new((start_pos, start_dir)),
        });

        let mut global_cache = HashMap::<(usize, (isize, isize)), usize>::new();
        global_cache.insert((start_pos, (start_dir)), 0);

        let mut best_score: Option<usize> = None;
        let mut positions_in_best_paths = HashSet::new();
        while let Some(cur) = pq.pop() {
            // exit if best score is reached
            if let Some(best_pos) = best_score {
                if cur.cost > best_pos {
                    continue;
                }
            }

            // check if at end
            if cur.pos == (end_x, end_y) {
                if let Some(best_score_u) = best_score {
                    if cur.cost < best_score_u {
                        best_score = None;
                        positions_in_best_paths.clear();
                        // panic!();
                    }
                }

                // dbg!(&cur);
                // println!("found end: {}", cur.cost);
                if best_score.is_none() {
                    best_score = Some(cur.cost);
                }

                let mut cur = &cur.visited;
                while let Some(prev) = &cur.previous {
                    positions_in_best_paths.insert(cur.current.0);
                    cur = prev.as_ref();
                }
                positions_in_best_paths.insert(cur.current.0);
                continue;
            }

            // go forward, if possible
            let next_pos = ((cur.pos.1 as isize + cur.dir.1) * stride as isize
                + (cur.pos.0 as isize + cur.dir.0)) as usize;
            if grid[next_pos] != b'#' && !cur.visited.contains(&(next_pos, cur.dir)) {
                let entry = global_cache
                    .entry((next_pos, cur.dir))
                    .or_insert(usize::MAX);
                if *entry < cur.cost + 1 {
                    continue;
                }

                *entry = cur.cost + 1;

                let next_pos_x = next_pos % stride;
                let next_pos_y = next_pos / stride;
                pq.push(MazeEntryGold {
                    pos: (next_pos_x, next_pos_y),
                    dir: cur.dir,
                    cost: cur.cost + 1,
                    distance_to_end: next_pos_x.abs_diff(end_x) + next_pos_y.abs_diff(end_y),
                    visited: cur.visited.clone().with_next((next_pos, cur.dir)),
                });
            }

            // try turning left and right
            let new_dirs = match cur.dir {
                (0, 1) => [(1, 0), (-1, 0)],
                (0, -1) => [(-1, 0), (1, 0)],
                (1, 0) => [(0, -1), (0, 1)],
                (-1, 0) => [(0, 1), (0, -1)],
                _ => unreachable!("invalid dir: {:?}", cur.dir),
            };
            let cur_pos = cur.pos.1 * stride + cur.pos.0;
            for new_dir in new_dirs {
                if cur.visited.contains(&(cur_pos, new_dir)) {
                    continue;
                }
                pq.push(MazeEntryGold {
                    pos: cur.pos,
                    dir: new_dir,
                    cost: cur.cost + 1000,
                    distance_to_end: cur.distance_to_end,
                    visited: cur.visited.clone(),
                });
            }
        }

        positions_in_best_paths.len()
    }
}

#[derive(Debug, Eq, PartialEq)]
struct MazeEntryGold {
    pos: (usize, usize),
    dir: (isize, isize),
    cost: usize,
    distance_to_end: usize,
    visited: LinkedListNode<(usize, (isize, isize))>,
}

impl PartialOrd for MazeEntryGold {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MazeEntryGold {
    fn cmp(&self, other: &Self) -> Ordering {
        // self.distance_to_end.cmp(&other.distance_to_end).reverse().then(self.cost.cmp(&other.cost).reverse())
        self.cost
            .cmp(&other.cost)
            .reverse()
            .then(self.distance_to_end.cmp(&other.distance_to_end).reverse())
        // (self.cost + self.distance_to_end * 1000).cmp(&(other.cost + other.distance_to_end * 1000)).reverse()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct LinkedListNode<T> {
    current: T,
    previous: Option<Rc<LinkedListNode<T>>>,
}

impl<T> LinkedListNode<T> {
    pub fn new(current: T) -> Self {
        Self {
            current,
            previous: None,
        }
    }

    pub fn with_next(self, next: T) -> Self {
        Self {
            current: next,
            previous: Some(Rc::new(self)),
        }
    }

    pub fn iter(&self) -> LinkedListIterator<T>
    where
        T: Clone,
    {
        LinkedListIterator {
            node: Some(Rc::new(self.clone())),
        }
    }

    pub fn len(&self) -> usize {
        let mut len = 1;
        let mut cur = self;
        while let Some(prev) = &cur.previous {
            len += 1;
            cur = prev;
        }
        len
    }

    pub fn contains(&self, value: &T) -> bool
    where
        T: PartialEq,
    {
        let mut cur = self;
        while let Some(prev) = &cur.previous {
            if &cur.current == value {
                return true;
            }
            cur = prev;
        }
        false
    }
}

struct LinkedListIterator<T> {
    node: Option<Rc<LinkedListNode<T>>>,
}

impl<T> Iterator for LinkedListIterator<T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node.take()?;
        self.node = node.previous.clone();
        Some(node.current.clone())
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(7036, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(98484, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(45, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(531, output);
}
