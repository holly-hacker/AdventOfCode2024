use core::str;
use std::collections::HashMap;

use fnv::FnvBuildHasher;
use petgraph::graph::NodeIndex;
use tinyvec::TinyVec;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 23;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let mut graph = petgraph::graph::UnGraph::<u16, ()>::new_undirected();
        let mut node_lookup = HashMap::<_, _, FnvBuildHasher>::default();
        let input = input.as_bytes();
        let line_count = (input.len() + 1) / 6;
        (0..line_count).for_each(|i| {
            let n1 = u16::from_ne_bytes(input[i * 6..][..2].try_into().unwrap());
            let n2 = u16::from_ne_bytes(input[i * 6 + 3..][..2].try_into().unwrap());

            let node1_key = *node_lookup.entry(n1).or_insert_with(|| graph.add_node(n1));
            let node2_key = *node_lookup.entry(n2).or_insert_with(|| graph.add_node(n2));

            graph.add_edge(node1_key, node2_key, ());
        });

        graph
            .node_indices()
            .map(|n| {
                let all_neighbours = &graph
                    .neighbors_undirected(n)
                    .filter(|&neighbour| neighbour.index() > n.index())
                    .collect::<TinyVec<[_; 16]>>();

                if all_neighbours.len() < 2 {
                    return 0;
                }

                let start_is_t = u16::to_ne_bytes(*graph.node_weight(n).unwrap())[0] == b't';

                // get all combinations of 2
                (0..all_neighbours.len())
                    .flat_map(move |i| (i..all_neighbours.len()).map(move |j| (i, j)))
                    .map(|(i, j)| [n, all_neighbours[i], all_neighbours[j]])
                    .filter(|a| {
                        start_is_t
                            || u16::to_ne_bytes(*graph.node_weight(a[1]).unwrap())[0] == b't'
                            || u16::to_ne_bytes(*graph.node_weight(a[2]).unwrap())[0] == b't'
                    })
                    .filter(|a| graph.contains_edge(a[1], a[2]))
                    .count()
            })
            .sum()
    }
}

impl SolutionGold<usize, String> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> String {
        let mut graph = petgraph::graph::UnGraph::<u16, ()>::new_undirected();
        let mut node_lookup = HashMap::<_, _, FnvBuildHasher>::default();
        let input = input.as_bytes();
        let line_count = (input.len() + 1) / 6;
        (0..line_count).for_each(|i| {
            let n1 = u16::from_ne_bytes(input[i * 6..][..2].try_into().unwrap());
            let n2 = u16::from_ne_bytes(input[i * 6 + 3..][..2].try_into().unwrap());

            let node1_key = *node_lookup.entry(n1).or_insert_with(|| graph.add_node(n1));
            let node2_key = *node_lookup.entry(n2).or_insert_with(|| graph.add_node(n2));

            graph.add_edge(node1_key, node2_key, ());
        });

        let mut overal_largest_group = None::<TinyVec<[NodeIndex; 16]>>;
        graph.node_indices().for_each(|leader_idx| {
            // assign the lowest value in a group to be the "leader" to prevent duplicate work
            // only look at nodes that have a higher value than the leader
            let leader_val = *graph.node_weight(leader_idx).unwrap();
            let neighbours = graph
                .neighbors_undirected(leader_idx)
                .filter(|&neighbour| graph.node_weight(neighbour).unwrap() > &leader_val)
                .collect::<TinyVec<[_; 16]>>();

            // for each group size, take a combination of neighbours and see if they are all connected
            // part 1 defined groups of size 3, so we can start from 2 required neighbours
            let min_size = overal_largest_group.as_ref().map(|g| g.len()).unwrap_or(2);
            let largest_group = (min_size..neighbours.len())
                .rev()
                .flat_map(|neighbour_count| {
                    // very hacky way of getting all combinations of N elements
                    (((1u32 << neighbour_count) - 1)..1 << (neighbour_count + 1))
                        .filter(|&num| num.count_ones() == neighbour_count as u32)
                        .map(|bitmap| {
                            (0..neighbours.len())
                                .filter(|i| bitmap & (1 << i) != 0)
                                .map(|i| neighbours[i])
                                .collect::<TinyVec<[_; 16]>>()
                        })
                        .filter(|group| {
                            // every item in the group must connect to all other items in the group
                            (0..group.len() - 1).all(|i| {
                                (i + 1..group.len())
                                    .all(|j| graph.contains_edge(group[i], group[j]))
                            })
                        })
                        .max_by_key(|group| group.len())
                })
                .next();

            if let Some(mut largest_group) = largest_group {
                if overal_largest_group.is_none()
                    || (overal_largest_group.is_some()
                        && overal_largest_group.as_ref().unwrap().len() < largest_group.len() + 1)
                {
                    largest_group.push(leader_idx);
                    overal_largest_group = Some(largest_group);
                }
            }
        });

        let mut vals = overal_largest_group
            .unwrap()
            .into_iter()
            .map(|idx| graph.node_weight(idx).unwrap())
            .map(|num| String::from_utf8_lossy(&num.to_ne_bytes()).to_string())
            .collect::<TinyVec<[_; 16]>>();

        vals.sort_unstable();

        vals.join(",")
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(7, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(1184, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!("co,de,ka,ta", output);
}

#[test]
#[allow(unused)]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!("hf,hz,lb,lm,ls,my,ps,qu,ra,uc,vi,xz,yv", output);
}
