use std::collections::{HashMap, HashSet, VecDeque};
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

fn bfs(graph: &Graph, start_node: i32) {
    let mut queue: VecDeque<i32> = VecDeque::new();
    let mut visited: HashSet<i32> = HashSet::new();
    queue.push_back(start_node);
    while let Some(node) = queue.pop_front() {
        if visited.insert(node) {
            println!("Visiting node {node}");
            if let Some(neighbors) = graph.get(&node) {
                for &neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        queue.push_back(neighbor)
                    }
                }
            }
        }
    }
}

fn main() {
    bfs(&GRAPH, 1)
}
