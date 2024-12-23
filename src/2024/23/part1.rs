use petgraph::{graph::UnGraph, visit::Bfs};
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> usize {
    let graph = parse_input(input);
    let mut visited = HashSet::new();
    for node in graph.node_indices() {
        if visited.contains(&node) {
            continue;
        }
        visited.insert(node);
        let mut bfs = Bfs::new(&graph, node);
    }
    0
}

fn parse_input(input: &str) -> UnGraph<&str, ()> {
    let mut graph = UnGraph::<&str, ()>::new_undirected();
    let mut node_map = HashMap::new();

    for line in input.lines() {
        let (source, target) = line.split_once('-').unwrap();

        let source_index = *node_map
            .entry(source)
            .or_insert_with(|| graph.add_node(source));
        let target_index = *node_map
            .entry(target)
            .or_insert_with(|| graph.add_node(target));

        graph.add_edge(source_index, target_index, ());
    }
    graph
}
