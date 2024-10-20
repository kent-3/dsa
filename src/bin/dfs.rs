use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

// Alias for an adjacency list representation
type Graph = HashMap<i32, Vec<i32>>;

static GRAPH: LazyLock<Graph> = LazyLock::new(|| {
    [
        (1, vec![2, 3]),
        (2, vec![4, 5]),
        (3, vec![6]),
        (4, vec![]),
        (5, vec![]),
        (6, vec![]),
    ]
    .iter()
    .cloned()
    .collect()
});

fn dfs_stack(graph: &Graph, start_node: i32) {
    let mut stack: Vec<i32> = Vec::new();
    let mut visited: HashSet<i32> = HashSet::new();
    stack.push(start_node);
    while let Some(node) = stack.pop() {
        if visited.insert(node) {
            println!("Visiting node {node}");
            if let Some(neighbors) = graph.get(&node) {
                // Equivalent to:
                // for neighbor in neighbors.iter().rev() {
                //     if !visited.contains(neighbor) {
                //         stack.push(*neighbor)
                //     }
                // }
                stack.extend(neighbors.iter().rev().filter(|&n| !visited.contains(n)))
            }
        }
    }
}

fn dfs_recursive(graph: &Graph, node: i32, visited: &mut HashSet<i32>) {
    if visited.insert(node) {
        println!("Visiting node {node}");
        if let Some(neighbors) = graph.get(&node) {
            // No iteration this time
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    dfs_recursive(graph, neighbor, visited)
                }
            }
        }
    }
}

fn main() {
    println!("Stack Method");
    dfs_stack(&GRAPH, 1);

    println!("Recursive Method");
    let mut visited: HashSet<i32> = HashSet::new();
    dfs_recursive(&GRAPH, 1, &mut visited);
}
