use petgraph::algo;
use petgraph::prelude::*;
use std::collections::HashMap;

#[allow(unused)]
pub fn run() {
    let input = include_str!("../input/12");

    let mut g = Graph::new_undirected();
    let mut indices = HashMap::new();

    for line in input.lines() {
        let mut iter = line.splitn(3, ' ');
        let from = iter.next().unwrap();
        let id = g.add_node(from);
        indices.insert(from, id);

        for to in iter.skip(1).next().unwrap().split(", ") {
            if let Some(&to_id) = indices.get(to) {
                g.add_edge(id, to_id, ());
            }
        }
    }

    let sccs = algo::kosaraju_scc(&g);
    for scc in &sccs {
        if scc.iter().any(|&n| n == indices["0"]) {
            println!("{}", scc.len());
            break;
        }
    }

    println!("{}", sccs.len());
}
