use core::str;
use std::collections::{BTreeSet, HashMap};

use petgraph::graph::NodeIndex;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 23;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let mut graph = petgraph::Graph::<u16, (), _, _>::new_undirected();
        let mut node_lookup = HashMap::new();
        input.lines().for_each(|l| {
            let n1 = u16::from_ne_bytes(l.as_bytes()[0..=1].try_into().unwrap());
            let n2 = u16::from_ne_bytes(l.as_bytes()[3..=4].try_into().unwrap());

            let node1_key = *node_lookup.entry(n1).or_insert_with(|| graph.add_node(n1));
            let node2_key = *node_lookup.entry(n2).or_insert_with(|| graph.add_node(n2));

            graph.add_edge(node1_key, node2_key, ());
        });

        graph
            .node_indices()
            .filter(|n| graph.neighbors_undirected(*n).count() >= 2)
            .flat_map(|n| {
                let all_neighbours = graph.neighbors_undirected(n).collect::<Vec<_>>();
                let all_neighbours = &all_neighbours;

                // get all combinations of 2
                (0..all_neighbours.len())
                    .flat_map(move |i| (1..all_neighbours.len()).map(move |j| (i, j)))
                    .map(|(i, j)| [all_neighbours[i], all_neighbours[j], n])
                    .filter(|a| a[0] != a[1])
                    .filter(|a| {
                        graph.contains_edge(a[0], a[1])
                            && graph.contains_edge(a[0], a[2])
                            && graph.contains_edge(a[1], a[2])
                    })
                    .map(|a| {
                        let mut a = a;
                        a.sort_unstable();
                        a
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
            .filter(|nodes| {
                nodes.iter().any(|n| {
                    let high_byte = u16::to_ne_bytes(*graph.node_weight(*n).unwrap())[0];
                    high_byte == b't'
                })
            })
            .count()
    }
}

impl SolutionGold<usize, String> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> String {
        let mut graph = petgraph::Graph::<u16, (), _, _>::new_undirected();
        let mut node_lookup = HashMap::new();
        input.lines().for_each(|l| {
            let n1 = u16::from_ne_bytes(l.as_bytes()[0..=1].try_into().unwrap());
            let n2 = u16::from_ne_bytes(l.as_bytes()[3..=4].try_into().unwrap());

            let node1_key = *node_lookup.entry(n1).or_insert_with(|| graph.add_node(n1));
            let node2_key = *node_lookup.entry(n2).or_insert_with(|| graph.add_node(n2));

            graph.add_edge(node1_key, node2_key, ());
        });

        let mut overal_largest_group = None::<Vec<NodeIndex>>;
        graph.node_indices().for_each(|leader_idx| {
            // assign the lowest value in a group to be the "leader" to prevent duplicate work
            // only look at nodes that have a higher value than the leader
            let leader_val = *graph.node_weight(leader_idx).unwrap();
            let neighbours = graph
                .neighbors_undirected(leader_idx)
                .filter(|&neighbour| graph.node_weight(neighbour).unwrap() > &leader_val)
                .collect::<Vec<_>>();

            // for each group size, take a combination of neighbours and see if they are all connected
            // TODO: can set min bound to overal largest group size here
            let largest_group = (2usize..=neighbours.len())
                .rev()
                .flat_map(|group_size| {
                    // very hacky way of getting all combinations of N elements
                    (0u32..1 << (group_size + 1))
                        .filter(|&num| num.count_ones() == group_size as u32)
                        .map(|bitmap| {
                            let mut group = vec![leader_idx];
                            group.extend(
                                (0..neighbours.len())
                                    .filter(|i| bitmap & (1 << i) != 0)
                                    .map(|i| neighbours[i]),
                            );
                            group
                        })
                        .filter(|group| {
                            // every item in the group must connect to all other items in the group
                            (0..group.len()).all(|i| {
                                (0..group.len())
                                    .all(|j| i == j || graph.contains_edge(group[i], group[j]))
                            })
                        })
                        .max_by_key(|group| group.len())
                })
                .next();

            if let Some(largest_group) = largest_group {
                if overal_largest_group.is_none()
                    || (overal_largest_group.is_some()
                        && overal_largest_group.as_ref().unwrap().len() < largest_group.len())
                {
                    overal_largest_group = Some(largest_group);
                }
            }
        });

        // verify solution
        let bla = overal_largest_group.as_ref().unwrap();
        for i in 0..bla.len() {
            for j in 0..bla.len() {
                if i != j {
                    debug_assert!(graph.contains_edge(bla[i], bla[j]));
                }
            }
        }

        let mut vals = overal_largest_group
            .unwrap()
            .into_iter()
            .map(|idx| graph.node_weight(idx).unwrap())
            .map(|num| String::from_utf8(num.to_ne_bytes().to_vec()).unwrap())
            .collect::<Vec<_>>();

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
