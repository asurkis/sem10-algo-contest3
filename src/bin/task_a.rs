use util::*;

fn main() {
    let mut lines = std::io::stdin().lines().map(|s| {
        s.unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>()
    });
    let nm = lines.next().unwrap();
    let n = nm[0];
    let m = nm[1];
    let mut edges = Vec::with_capacity(n);
    for _ in 0..n {
        let mut ae = lines.next().unwrap();
        ae.pop().unwrap();
        edges.push(ae);
    }

    let answer = solve(n, m, &edges);
    println!("{}", answer.len());
    for [u, v] in answer {
        println!("{u} {v}");
    }
}

fn solve(n: usize, m: usize, edges: &[Vec<usize>]) -> Vec<[usize; 2]> {
    assert_eq!(n, edges.len());
    let mut g = Graph::new(2 * (n + m) + 2);
    let s = 2 * (n + m);
    let t = s + 1;
    let mut init_edges = Vec::new();
    for v in 0..n {
        for &u in &edges[v] {
            let pos = g.add_edge(v, n + u - 1);
            init_edges.push(pos);
        }
    }

    for v in 0..n {
        let u = n + m + v;
        g.add_edge(s, u);
        g.add_edge(u, v);
    }

    for v in 0..m {
        let v = n + v;
        let u = n + m + v;
        g.add_edge(v, u);
        g.add_edge(u, t);
    }

    while g.mark_subflow(s, t) {}

    let mut answer = Vec::new();
    for i in init_edges {
        let e = g.get_edge(i);
        if e.capacity != 0 {
            continue;
        }
        answer.push([e.node1 + 1, e.node2 - n + 1]);
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // 2 2
        // 1 2 0
        // 2 0
        // ========
        // 2
        // 1 1
        // 2 2
        let edges = vec![vec![1, 2], vec![2]];
        let actual = solve(2, 2, &edges);
        let expected = vec![[1, 1], [2, 2]];
        assert_eq!(expected, actual);
    }

    #[test]
    fn test2() {
        // 5 5
        // 1 2 3 0
        // 1 4 5 0
        // 1 2 0
        // 3 4 5 0
        // 1 0
        // ========
        // 5
        // 5 1
        // 3 2
        // 1 3
        // 4 4
        // 2 5
        let edges = vec![
            vec![1, 2, 3],
            vec![1, 4, 5],
            vec![1, 2],
            vec![3, 4, 5],
            vec![1],
        ];
        let actual = solve(5, 5, &edges);
        let expected = vec![[5, 1], [3, 2], [1, 3], [4, 4], [2, 5]];
        assert_eq!(expected.len(), actual.len());
        // assert_eq!(expected, actual);
    }
}

#[allow(unused)]
mod util {
    use std::collections::VecDeque;

    #[cfg(test)]
    #[macro_export]
    macro_rules! debug {
        ($($($val:expr),+);*) => {
            $(
                eprint!("[{}:{}]", file!(), line!());
                $(
                    eprint!("  {} = {:?}", stringify!($val), $val);
                )*
                eprintln!();
            )*
        };
    }

    #[cfg(not(test))]
    #[macro_export]
    macro_rules! debug {
        ($($($val:expr),+);*) => {};
    }

    const fn ilog2_acc(x: usize, acc: u32) -> u32 {
        if x == 1 {
            acc
        } else {
            ilog2_acc(x >> 1, acc + 1)
        }
    }

    pub const fn ilog2(x: usize) -> u32 {
        if x == 0 {
            panic!();
        }
        ilog2_acc(x, 0)
    }

    pub const fn ceil2(x: usize) -> usize {
        if x == 0 {
            1
        } else {
            1 << ilog2(2 * x - 1)
        }
    }

    pub fn calc_zfun(s: &[char], z: &mut [usize]) {
        let n = s.len();
        assert_eq!(n, z.len());
        z[0] = 0;
        let mut l = 1;
        let mut r = 1;
        z[1] = r - l;
        for i in 1..n {
            let mut k = 0;
            if i < r {
                // s[i..r] = s[i - l..r - l]
                k = z[i - l].min(r - i);
            }
            while i + k < n && s[k] == s[i + k] {
                k += 1;
            }
            z[i] = k;
            if i + k > r {
                l = i;
                r = i + k;
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Edge {
        pub node1: usize,
        pub node2: usize,
        pub capacity: u32,
    }

    #[derive(Debug, Clone)]
    pub struct Graph {
        edge_heap: Vec<Edge>,
        edge_idx: Vec<Vec<usize>>,
    }

    impl Graph {
        pub fn new(n_nodes: usize) -> Self {
            Self {
                edge_heap: Vec::new(),
                edge_idx: vec![Vec::new(); n_nodes],
            }
        }

        pub fn n_nodes(&self) -> usize {
            self.edge_idx.len()
        }

        pub fn n_edges(&self) -> usize {
            self.edge_heap.len()
        }

        pub fn add_edge(&mut self, node1: usize, node2: usize) -> usize {
            let pos = self.n_edges();
            self.edge_idx[node1].push(pos);
            self.edge_idx[node2].push(pos + 1);
            self.edge_heap.push(Edge {
                node1,
                node2,
                capacity: 1,
            });
            self.edge_heap.push(Edge {
                node1: node2,
                node2: node1,
                capacity: 0,
            });
            pos
        }

        pub fn get_edge(&self, i: usize) -> Edge {
            self.edge_heap[i]
        }

        pub fn mark_subflow(&mut self, s: usize, t: usize) -> bool {
            debug_assert_ne!(s, t);
            let mut last_edge = vec![usize::MAX; self.n_nodes()];
            let mut queue = VecDeque::new();
            queue.push_back(s);
            last_edge[s] = usize::MAX - 1;
            while !queue.is_empty() {
                let v = queue.pop_front().unwrap();
                for &ei in &self.edge_idx[v] {
                    let e = self.edge_heap[ei];
                    if e.capacity == 0 {
                        continue;
                    }
                    if last_edge[e.node2] != usize::MAX {
                        continue;
                    }
                    last_edge[e.node2] = ei;
                    queue.push_back(e.node2);
                }
            }

            if last_edge[t] == usize::MAX {
                return false;
            }

            let mut pos = t;
            while pos != s {
                let ei = last_edge[pos];
                let cap = self.edge_heap[ei].capacity;
                self.edge_heap[ei].capacity -= cap;
                self.edge_heap[ei ^ 1].capacity += cap;
                pos = self.edge_heap[ei].node1;
            }
            true
        }
    }
}
