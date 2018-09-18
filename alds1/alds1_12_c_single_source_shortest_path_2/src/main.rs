// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_12_C

// Original code from https://doc.rust-lang.org/std/collections/binary_heap/

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::usize;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug)]
struct Edge {
    node: usize,
    cost: usize,
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(State { cost: 0, position: start });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal { return Some(cost); }

        // Important as we may have already found a better way
        if cost > dist[position] { continue; }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position] {
            let next = State { cost: cost + edge.cost, position: edge.node };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).ok();
    let n = n.trim().parse::<usize>().unwrap();

    let mut g: Vec<Vec<Edge>> = vec![vec![]; n];
    for _ in 0..n {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).ok();
        let buffer: Vec<usize> = buffer
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();  
        let v1 = buffer[0];
        let degree = buffer[1];
        for i in (0..degree*2).step_by(2) {
            let v2 = buffer[2+i];
            let cost = buffer[3+i];
            g[v1].push(Edge { node: v2, cost: cost });
        }
    }

    for i in 0..n {
        println!("{} {}", i, shortest_path(&g, 0, i).unwrap());
    }
}
