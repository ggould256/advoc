
use std::collections::{HashMap, HashSet};

use log::debug;

use nalgebra::Vector3;
use regex::Regex;

use crate::common::parsing::read_regex_records;

type Xyz = Vector3<i64>;

fn read_input(source: Option<String>) -> Vec<Xyz> {
    let re = Regex::new(r"(\d+),(\d+),(\d+)").unwrap();
    let point_records = read_regex_records(source, re);
    let mut result: Vec<Xyz> = Vec::new();
    for record_strings in point_records {
        assert_eq!(record_strings.len(), 4);
        let record: Xyz = Xyz::new(
            record_strings[1].parse::<i64>().unwrap(),
            record_strings[2].parse::<i64>().unwrap(),
            record_strings[3].parse::<i64>().unwrap(),
        );
        result.push(record);
    }
    debug!("Read {} points", result.len());
    result
}

fn magnitude(xyz: Xyz) -> i64 {
    // This should be a nalgebra builtin?
    xyz.x * xyz.x + xyz.y * xyz.y + xyz.z * xyz.z
}

type ConnectionsList = Vec<(usize, usize, i64)>;

/// Return all of the pairwise connections (from, to, cost) sorted by cost.  `from` < `to`.
fn compute_distances(points: &[Xyz]) -> ConnectionsList {
    let mut result: ConnectionsList = ConnectionsList::new();
    for from_idx in 0..points.len() {
        let from = points[from_idx];
        #[allow(clippy::needless_range_loop)]
        for to_idx in (from_idx + 1)..points.len() {
            let to = points[to_idx];
            let d2 = magnitude(from - to);
            result.push((from_idx, to_idx, d2));
        }
    }
    result.sort_by_key(|(_, _, cost)| *cost);
    debug!("Computed {} distances", result.len());
    debug!("Smallest {:?} largest {:?}", result[0], result.last().unwrap());
    result
}

/// Make the at most n connections of least cost.  Return the resulting subgraphs, largest to
/// smallest.
fn make_n_connections(points: &[Xyz], possible_connections: &ConnectionsList, n: usize)
-> (Vec<HashSet<usize>>, (usize, usize)) {
    let mut next_subgraph_id: usize = 1;
    let mut node_to_subgraph_id: HashMap<usize, usize> = HashMap::new();
    let mut connections_made: usize = 0;
    let mut last_connection: (usize, usize) = (0, 0);
    for (f, t, c) in possible_connections {
        debug!("{} connections so far; Considering connecting from {} to {} (cost {})",
               connections_made, f, t, c);
        // Four possibilities:
        // (1) f and t are in the same subgraph; nothing happens.
        // (2) f and t are in different subgraphs; merge them.
        // (3) f and t are both absent; add them as a new subgraph.
        // (4) Only one is present; add the other to its subgraph.
        if !node_to_subgraph_id.contains_key(f) && !node_to_subgraph_id.contains_key(t) {
            let id = next_subgraph_id;
            next_subgraph_id += 1;
            node_to_subgraph_id.insert(*f, id);
            node_to_subgraph_id.insert(*t, id);
            debug!("Connected new neighbors {} -> {} as graph ID {}", f, t, id);
        } else if node_to_subgraph_id.contains_key(f) && !node_to_subgraph_id.contains_key(t) {
            node_to_subgraph_id.insert(*t, node_to_subgraph_id[f]);
            debug!("Connected new {} to existing {} of graph {}", t, f, node_to_subgraph_id[f]);
        } else if !node_to_subgraph_id.contains_key(f) && node_to_subgraph_id.contains_key(t) {
            node_to_subgraph_id.insert(*f, node_to_subgraph_id[t]);
            debug!("Connected new {} to existing {} of graph {}", f, t, node_to_subgraph_id[t]);
        } else {
            debug!("Considering {} (graph {}) -> {} (graph {})", f, node_to_subgraph_id[f], t, node_to_subgraph_id[t]);
            if node_to_subgraph_id[t] == node_to_subgraph_id[f] {
                debug!("Discarded {} -> {} as both are in graph {}", f, t, node_to_subgraph_id[t]);
                // We should `continue` here, if this doesn't count as a connection; the rules are
                // not clear.
            } else {
                // Merge subgraphs:  Relabel all nodes in graph 't' as being in graph 'f'
                let t_graph = node_to_subgraph_id[t];
                let f_graph = node_to_subgraph_id[f];
                debug!("Connection {} -> {} merges graph {} into {}", f, t, t_graph, f_graph);
                let keys: Vec<usize> = node_to_subgraph_id.keys().copied().collect();
                for k in keys {
                    if node_to_subgraph_id[&k] == t_graph {
                        node_to_subgraph_id.insert(k, f_graph);
                    }
                }
            }
        }
        last_connection = (*f, *t);
        connections_made += 1;
        if connections_made >= n { break; }
        if node_to_subgraph_id.len() == points.len() { break; }
    }
    let mut result: Vec<HashSet<usize>> = Vec::new();
    let subgraphs = node_to_subgraph_id;
    debug!("Found subgraph membership map: {:?}", subgraphs);
    debug!("Last connection was: {} {:?} -> {} {:?}",
           last_connection.0, points[last_connection.0],
           last_connection.1, points[last_connection.1]);
    for i in 1..next_subgraph_id {
        let mut subgraph: HashSet<usize> = HashSet::new();
        for (&point_idx, &subgraph_idx) in &subgraphs {
            if subgraph_idx == i { subgraph.insert(point_idx); }
        }
        result.push(subgraph);
    }
    result.sort_by_key(|s| -(s.len() as i64));
    debug!("result {:?}", result);
    (result, last_connection)
}


pub fn solution_a_example(source: Option<String>) -> i64 {
    let points = &read_input(source);
    let distances = compute_distances(points);
    let (subgraphs, _) = make_n_connections(points,&distances, 10);
    let result = (subgraphs.iter().take(3).map(|s| s.len()).product::<usize>()) as i64;
    result
}

pub fn solution_a(source: Option<String>) -> i64 {
    let points = &read_input(source);
    let distances = compute_distances(points);
    let (subgraphs, _) = make_n_connections(points, &distances, 1000);
    let result = (subgraphs.iter().take(3).map(|s| s.len()).product::<usize>()) as i64;
    result
}

pub fn solution_b(source: Option<String>) -> i64 {
    let points = &read_input(source);
    let distances = compute_distances(points);
    let (_, last_two_points) = make_n_connections(points, &distances, 9999999);
    
    points[last_two_points.0][0] * points[last_two_points.1][0]
}

#[cfg(test)]
mod tests {
    use super::*;

    use const_format::concatcp;
    use log::info;
    use std::fs::File;

    const DAY: &str = "8";
    const EXAMPLE_A_DATA: &str = concatcp!("data/2025/day", DAY, "a_example.txt");
    const EXAMPLE_B_DATA: &str = concatcp!("data/2025/day", DAY, "a_example.txt");
    const INPUT_A_DATA: &str = concatcp!("inputs/2025/day", DAY, "_test.txt");
    const INPUT_B_DATA: &str = concatcp!("inputs/2025/day", DAY, "_test.txt");

    #[test]
    fn test_example() {
        assert_eq!(solution_a_example(Some(EXAMPLE_A_DATA.to_string())), 40);
    }

    #[test]
    fn test_test_a() {
        if File::open(INPUT_A_DATA).is_err() {
            info!("Skipping test that requires input not in repository");
            return;
        }
        assert_eq!(solution_a(Some(INPUT_A_DATA.to_string())), 123234);
    }

    #[test]
    fn test_example_b() {
        assert_eq!(solution_b(Some(EXAMPLE_B_DATA.to_string())), 25272);
    }

    #[test]
    fn test_test_b() {
        if File::open(INPUT_B_DATA).is_err() {
            info!("Skipping test that requires input not in repository");
            return;
        }
        assert_eq!(solution_b(Some(INPUT_B_DATA.to_string())), 9259958565);
    }
}
