use priority_queue::PriorityQueue;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fs,
};

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

fn parse_input(input: &str) -> Graph {
    let mut result: Graph = HashMap::new();

    for line in input.lines() {
        let (node, successors) = line.split_once(':').unwrap();

        let successors = successors.split_ascii_whitespace().collect::<Vec<_>>();

        result.entry(node).or_default().extend(&successors);

        for successor in successors {
            result.entry(successor).or_default().push(node);
        }
    }

    result
}

fn get_reachable<'a>(graph: &Graph<'a>, source: &'a str) -> HashSet<&'a str> {
    // perform a DFS from source
    // panics on bad inputs

    let mut stack = vec![source];
    let mut seen = HashSet::from([source]);

    while let Some(node) = stack.pop() {
        for &successor in &graph[node] {
            if !seen.contains(successor) {
                stack.push(successor);
                seen.insert(successor);
            }
        }
    }

    seen
}

struct MinCutInfo<'a> {
    min_cut_size: usize,
    partition_a: Vec<&'a str>,
    partition_b: Vec<&'a str>,
}

// this documentation helped:
// https://networkx.org/documentation/stable/_modules/networkx/algorithms/connectivity/stoerwagner.html#stoer_wagner
// assumes graph is connected and has at least 2 nodes
fn stoer_wagner<'a>(graph: &Graph<'a>) -> MinCutInfo<'a> {
    let mut weighted_graph: HashMap<&str, RefCell<HashMap<&str, usize>>> = graph
        .iter()
        .map(|(&node, successors)| {
            (
                node,
                successors
                    .iter()
                    .map(|&succ| (succ, 1))
                    .collect::<HashMap<_, _>>()
                    .into(),
            )
        })
        .collect();

    let mut min_cut_size = usize::MAX;
    let mut contractions = Vec::new();
    let mut best_phase = 0;

    for i in 0.. {
        // get an arbitrary element of the graph
        let (&u, u_successors) = weighted_graph.iter().next().unwrap();
        let mut partition_a = HashSet::from([u]);

        let mut u = u;

        let mut heap: PriorityQueue<&str, usize> = u_successors
            .borrow()
            .iter()
            .map(|(&succ, &weight)| (succ, weight))
            .collect();

        for _ in 0..graph.len() - i - 2 {
            (u, _) = heap.pop().unwrap();

            partition_a.insert(u);
            for (&successor, &weight) in weighted_graph[u].borrow().iter() {
                if !partition_a.contains(successor)
                    && !heap.change_priority_by(successor, |p| *p += weight)
                {
                    heap.push(successor, weight);
                }
            }
        }

        let u = u;
        let (v, w) = heap.pop().unwrap();

        if w < min_cut_size {
            min_cut_size = w;
            best_phase = i;
        }

        contractions.push((u, v));

        if i == graph.len() - 2 {
            break;
        }
        // contract u and v into 1 node
        let mut combined_succs = weighted_graph[u].borrow_mut();
        combined_succs.remove(v);
        for (&successor, &weight) in weighted_graph[v].borrow().iter() {
            if successor == u {
                continue;
            }

            let combined_weight = combined_succs.entry(successor).or_default();

            *combined_weight += weight;

            let mut succ_succs = weighted_graph[successor].borrow_mut();
            succ_succs.insert(u, *combined_weight);
            succ_succs.remove(v);
        }
        drop(combined_succs);

        weighted_graph.remove(v);
    }

    let mut contractions_graph: Graph = HashMap::new();
    for &(u, v) in &contractions[0..best_phase] {
        contractions_graph.entry(u).or_default().push(v);
        contractions_graph.entry(v).or_default().push(u);
    }

    let source = contractions[best_phase].1;

    // if source not in the graph then add it with 0 edges
    contractions_graph.entry(source).or_default();

    let partition_a = get_reachable(&contractions_graph, source);
    let partition_b = graph
        .keys()
        .copied()
        .filter(|&node| !partition_a.contains(node))
        .collect();

    MinCutInfo {
        min_cut_size,
        partition_a: partition_a.into_iter().collect(),
        partition_b,
    }
}

fn main() {
    let file_contents = fs::read_to_string("input.txt").unwrap();

    let graph = parse_input(&file_contents);

    let MinCutInfo {
        min_cut_size,
        partition_a,
        partition_b,
    } = stoer_wagner(&graph);

    assert_eq!(min_cut_size, 3);

    println!("{}", partition_a.len() * partition_b.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "jqt: rhn xhk nvd\n\
                              rsh: frs pzl lsr\n\
                              xhk: hfx\n\
                              cmg: qnr nvd lhk bvb\n\
                              rhn: xhk bvb hfx\n\
                              bvb: xhk hfx\n\
                              pzl: lsr hfx nvd\n\
                              qnr: nvd\n\
                              ntq: jqt hfx bvb xhk\n\
                              nvd: lhk\n\
                              lsr: lhk\n\
                              rzs: qnr cmg lsr rsh\n\
                              frs: qnr lhk lsr";

    #[test]
    fn test_parsing() {
        let graph = parse_input(TEST_INPUT);
        for (node, successors) in graph {
            println!("{node}: {successors:?}");
        }
    }

    #[test]
    fn test_part1() {
        let graph = parse_input(TEST_INPUT);
        let MinCutInfo {
            min_cut_size,
            partition_a,
            partition_b,
        } = stoer_wagner(&graph);

        assert_eq!(min_cut_size, 3);
        assert_eq!(partition_a.len() * partition_b.len(), 54);
    }
}
