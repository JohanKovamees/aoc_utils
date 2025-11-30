use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use priority_queue::PriorityQueue;

/// Breadth-first search: returns distance map from `start`.
pub fn bfs<T, F, I>(start: T, mut neighbors: F) -> HashMap<T, usize>
where
    T: Eq + Hash + Copy,
    F: FnMut(T) -> I,
    I: IntoIterator<Item = T>,
{
    let mut dist = HashMap::new();
    let mut q = VecDeque::new();

    dist.insert(start, 0);
    q.push_back(start);

    while let Some(cur) = q.pop_front() {
        let d = dist[&cur];
        for nb in neighbors(cur) {
            if !dist.contains_key(&nb) {
                dist.insert(nb, d + 1);
                q.push_back(nb);
            }
        }
    }

    dist
}

/// Depth-first search (non-recursive).
pub fn dfs<T, F, I>(start: T, mut neighbors: F) -> Vec<T>
where
    T: Eq + Hash + Copy,
    F: FnMut(T) -> I,
    I: IntoIterator<Item = T>,
{
    let mut visited = HashSet::new();
    let mut stack = vec![start];
    let mut order = Vec::new();

    while let Some(cur) = stack.pop() {
        if !visited.insert(cur) {
            continue;
        }
        order.push(cur);
        for nb in neighbors(cur) {
            if !visited.contains(&nb) {
                stack.push(nb);
            }
        }
    }

    order
}

/// Dijkstra: returns (distance map, previous-node map).
pub fn dijkstra<T, F, I>(start: T, mut neighbors: F) -> (HashMap<T, i64>, HashMap<T, T>)
where
    T: Eq + Hash + Copy,
    F: FnMut(T) -> I,
    I: IntoIterator<Item = (T, i64)>, // (neighbor, cost)
{
    let mut dist: HashMap<T, i64> = HashMap::new();
    let mut prev: HashMap<T, T> = HashMap::new();
    let mut pq = PriorityQueue::new();

    dist.insert(start, 0);
    pq.push(start, std::cmp::Reverse(0_i64));

    while let Some((u, std::cmp::Reverse(d))) = pq.pop() {
        if d > dist[&u] {
            continue; // outdated entry
        }

        for (v, w) in neighbors(u) {
            let nd = d + w;
            if dist.get(&v).map_or(true, |&old| nd < old) {
                dist.insert(v, nd);
                prev.insert(v, u);
                pq.push(v, std::cmp::Reverse(nd));
            }
        }
    }

    (dist, prev)
}

/// Reconstruct path from start to `end` using `prev` map returned by dijkstra.
pub fn reconstruct_path<T>(prev: &HashMap<T, T>, end: T) -> Vec<T>
where
    T: Eq + Hash + Copy,
{
    let mut path = Vec::new();
    let mut cur = end;
    path.push(cur);

    while let Some(&p) = prev.get(&cur) {
        cur = p;
        path.push(cur);
    }

    path.reverse();
    path
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // ---- bfs tests ----

    #[test]
    fn bfs_on_simple_line_graph() {
        // 0 - 1 - 2 - 3
        let neighbors = |n: i32| -> Vec<i32> {
            match n {
                0 => vec![1],
                1 => vec![0, 2],
                2 => vec![1, 3],
                3 => vec![2],
                _ => vec![],
            }
        };

        let dist = bfs(0, neighbors);

        let mut expected = HashMap::new();
        expected.insert(0, 0);
        expected.insert(1, 1);
        expected.insert(2, 2);
        expected.insert(3, 3);

        assert_eq!(dist, expected);
    }

    #[test]
    fn bfs_finds_shortest_paths_in_unweighted_graph() {
        // Graph:
        //  1
        //  |
        //  0 - 2 - 3
        //   \      /
        //     4 --
        //
        // shortest from 0:
        // 1:1, 2:1, 4:1, 3:2
        let neighbors = |n: i32| -> Vec<i32> {
            match n {
                0 => vec![1, 2, 4],
                1 => vec![0],
                2 => vec![0, 3],
                3 => vec![2, 4],
                4 => vec![0, 3],
                _ => vec![],
            }
        };

        let dist = bfs(0, neighbors);

        assert_eq!(dist.get(&0), Some(&0));
        assert_eq!(dist.get(&1), Some(&1));
        assert_eq!(dist.get(&2), Some(&1));
        assert_eq!(dist.get(&4), Some(&1));
        assert_eq!(dist.get(&3), Some(&2));
    }

    #[test]
    fn bfs_on_isolated_node() {
        let neighbors = |_n: i32| -> Vec<i32> { vec![] };
        let dist = bfs(42, neighbors);

        assert_eq!(dist.len(), 1);
        assert_eq!(dist.get(&42), Some(&0));
    }

    // ---- dfs tests ----

    #[test]
    fn dfs_order_with_deterministic_neighbors() {
        // Directed-like graph (via neighbor function):
        // A -> B, C
        // B -> D
        // C -> D
        //
        // Stack-based DFS with neighbors in the given order.
        // Start: 'A'
        //
        // stack process:
        //   start: ['A']
        //   pop A -> push neighbors B, C => stack [B, C]
        //   pop C -> push neighbor D => stack [B, D]
        //   pop D -> no neighbors => stack [B]
        //   pop B -> neighbor D already visited
        //
        // visit order: A, C, D, B
        let neighbors = |n: char| -> Vec<char> {
            match n {
                'A' => vec!['B', 'C'],
                'B' => vec!['D'],
                'C' => vec!['D'],
                'D' => vec![],
                _ => vec![],
            }
        };

        let order = dfs('A', neighbors);
        assert_eq!(order, vec!['A', 'C', 'D', 'B']);
    }

    #[test]
    fn dfs_on_isolated_node() {
        let neighbors = |_n: i32| -> Vec<i32> { vec![] };
        let order = dfs(7, neighbors);
        assert_eq!(order, vec![7]);
    }

    // ---- dijkstra tests ----

    #[test]
    fn dijkstra_on_simple_weighted_graph() {
        // Weighted graph:
        //
        // A -1-> B -2-> D
        //  \           ^
        //   5         /
        //    \       / 1
        //     v     /
        //      C -- 
        //
        // Paths from A:
        //   A -> B -> D: 1 + 2 = 3 (shortest)
        //   A -> C -> D: 5 + 1 = 6
        //
        let neighbors = |n: char| -> Vec<(char, i64)> {
            match n {
                'A' => vec![('B', 1), ('C', 5)],
                'B' => vec![('D', 2)],
                'C' => vec![('D', 1)],
                'D' => vec![],
                _ => vec![],
            }
        };

        let (dist, prev) = dijkstra('A', neighbors);

        assert_eq!(dist.get(&'A'), Some(&0));
        assert_eq!(dist.get(&'B'), Some(&1));
        assert_eq!(dist.get(&'D'), Some(&3));

        // prev should show that the best path is A -> B -> D
        assert_eq!(prev.get(&'B'), Some(&'A'));
        assert_eq!(prev.get(&'D'), Some(&'B'));

        // reconstruct the path A -> D
        let path = reconstruct_path(&prev, 'D');
        assert_eq!(path, vec!['A', 'B', 'D']);
    }

    #[test]
    fn dijkstra_handles_outdated_queue_entries() {
        // Graph where we first find a worse path to C, then a better one.
        //
        // A -> B (cost 5)
        // A -> C (cost 10)
        // B -> C (cost 1)  => better path A->B->C with cost 6
        //
        let neighbors = |n: char| -> Vec<(char, i64)> {
            match n {
                'A' => vec![('B', 5), ('C', 10)],
                'B' => vec![('C', 1)],
                'C' => vec![],
                _ => vec![],
            }
        };

        let (dist, prev) = dijkstra('A', neighbors);

        assert_eq!(dist.get(&'A'), Some(&0));
        assert_eq!(dist.get(&'B'), Some(&5));
        assert_eq!(dist.get(&'C'), Some(&6));

        assert_eq!(prev.get(&'B'), Some(&'A'));
        assert_eq!(prev.get(&'C'), Some(&'B'));

        let path = reconstruct_path(&prev, 'C');
        assert_eq!(path, vec!['A', 'B', 'C']);
    }

    #[test]
    fn dijkstra_with_isolated_start() {
        let neighbors = |_n: i32| -> Vec<(i32, i64)> { vec![] };
        let (dist, prev) = dijkstra(0, neighbors);

        assert_eq!(dist.len(), 1);
        assert_eq!(dist.get(&0), Some(&0));
        assert!(prev.is_empty());
    }

    // ---- reconstruct_path tests ----

    #[test]
    fn reconstruct_path_trivial() {
        // No parents in map, path should just be [end]
        let prev: HashMap<i32, i32> = HashMap::new();
        let path = reconstruct_path(&prev, 99);
        assert_eq!(path, vec![99]);
    }

    #[test]
    fn reconstruct_path_longer_chain() {
        // prev: 3 <- 2 <- 1
        let mut prev = HashMap::new();
        prev.insert(3, 2);
        prev.insert(2, 1);

        let path = reconstruct_path(&prev, 3);
        assert_eq!(path, vec![1, 2, 3]);
    }
}
