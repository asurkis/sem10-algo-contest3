use crate::util::Graph;

fn main() {
    let mut lines = std::io::stdin().lines().map(|s| s.unwrap());
    let nm_str = lines.next().unwrap();
    let (n_str, _) = nm_str.trim().split_once(' ').unwrap();
    let n = n_str.parse().unwrap();
    let strings: Vec<String> = lines.take(n).collect();
    let trimmed: Vec<&str> = strings.iter().map(|s| s.trim()).collect();
    let answer = solve(&trimmed);
    if answer {
        println!("Valid");
    } else {
        println!("Invalid");
    }
}

fn solve(lines: &[&str]) -> bool {
    let n = lines.len();
    debug_assert_ne!(0, n);
    let m = lines[0].len();
    #[cfg(test)]
    for i in 0..n {
        debug_assert_eq!(m, lines[i].len());
    }

    let mut total_cap = 0;
    let mut g = Graph::new(2 * n * m + 2);
    let s = 2 * n * m;
    let t = s + 1;
    for i in 0..n {
        for (j, c) in lines[i].chars().enumerate() {
            let wrap = i * m + j;
            let cap = match c {
                '.' => 0,
                'H' => 1,
                'O' => 2,
                'N' => 3,
                'C' => 4,
                _ => panic!("Unexpected char {c}"),
            };
            total_cap += cap;
            g.add_edge(2 * wrap, 2 * wrap + 1, cap);

            if (i + j) % 2 == 0 {
                g.add_edge(s, 2 * wrap, cap);
            } else {
                g.add_edge(2 * wrap + 1, t, cap);
            }
        }
    }

    for i in 0..n {
        for j in 1..m {
            let wrap1 = i * m + j - 1;
            let wrap2 = i * m + j;
            if (i + j) % 2 == 0 {
                g.add_edge(2 * wrap2 + 1, 2 * wrap1, 1);
            } else {
                g.add_edge(2 * wrap1 + 1, 2 * wrap2, 1);
            }
        }
    }

    for i in 1..n {
        for j in 0..m {
            let wrap1 = (i - 1) * m + j;
            let wrap2 = i * m + j;
            if (i + j) % 2 == 0 {
                g.add_edge(2 * wrap2 + 1, 2 * wrap1, 1);
            } else {
                g.add_edge(2 * wrap1 + 1, 2 * wrap2, 1);
            }
        }
    }

    let mut achieved = 0;
    while {
        let flow = g.mark_subflow(s, t);
        achieved += flow;
        flow != 0
    } {}

    debug!(achieved, total_cap);
    2 * achieved == total_cap
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // 3 4
        // HOH.
        // NCOH
        // OO..
        // ========
        // Valid
        let answer = solve(&["HOH.", "NCOH", "OO.."]);
        assert!(answer);
    }

    #[test]
    fn test2() {
        // 3 4
        // HOH.
        // NCOH
        // OONH
        // ========
        // Invalid
        let answer = solve(&["HOH.", "NCOH", "OONH"]);
        assert!(!answer);
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

    pub fn calc_zfun_inplace(s: &[char], z: &mut [usize]) {
        let n = s.len();
        assert_eq!(n, z.len());
        z[0] = n;
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

    pub fn calc_zfun(s: &[char]) -> Vec<usize> {
        let mut zfun = vec![0; s.len()];
        calc_zfun_inplace(s, &mut zfun);
        zfun
    }

    pub type Capacity = u32;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Edge {
        pub node1: usize,
        pub node2: usize,
        pub capacity: Capacity,
        pub is_real: bool,
    }

    #[derive(Debug, Clone)]
    pub struct Graph {
        edge_heap: Vec<Edge>,
        edge_idx: Vec<Vec<usize>>,
        last_edge_buf: Vec<usize>,
    }

    impl Graph {
        pub fn new(n_nodes: usize) -> Self {
            Self {
                edge_heap: Vec::new(),
                edge_idx: vec![Vec::new(); n_nodes],
                last_edge_buf: vec![0; n_nodes],
            }
        }

        pub fn n_nodes(&self) -> usize {
            self.edge_idx.len()
        }

        pub fn n_edges(&self) -> usize {
            self.edge_heap.len()
        }

        pub fn add_edge(&mut self, node1: usize, node2: usize, capacity: Capacity) -> usize {
            self.add_one_edge(node2, node1, 0, false);
            self.add_one_edge(node1, node2, capacity, true)
        }

        fn add_one_edge(
            &mut self,
            node1: usize,
            node2: usize,
            capacity: Capacity,
            is_real: bool,
        ) -> usize {
            let pos = self.n_edges();
            self.edge_idx[node1].push(pos);
            self.edge_heap.push(Edge {
                node1,
                node2,
                capacity,
                is_real,
            });
            pos
        }

        pub fn edge(&self, i: usize) -> Edge {
            self.edge_heap[i]
        }

        pub fn mark_subflow(&mut self, s: usize, t: usize) -> Capacity {
            debug_assert_ne!(s, t);
            let last_edge = &mut self.last_edge_buf;
            last_edge.fill(usize::MAX);
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
                return 0;
            }

            let mut flow = Capacity::MAX;
            let mut pos = t;
            while pos != s {
                let ei = last_edge[pos];
                let cap = self.edge_heap[ei].capacity;
                flow = flow.min(cap);
                pos = self.edge_heap[ei].node1;
            }

            let mut pos = t;
            while pos != s {
                let ei = last_edge[pos];
                self.edge_heap[ei].capacity -= flow;
                self.edge_heap[ei ^ 1].capacity += flow;
                pos = self.edge_heap[ei].node1;
            }
            flow
        }

        pub fn mark_reachable(&self, s: usize, out: &mut [bool], filter: &impl Fn(&Edge) -> bool) {
            if out[s] {
                return;
            }
            out[s] = true;
            for &ei in &self.edge_idx[s] {
                let e = &self.edge_heap[ei];
                if !filter(e) {
                    continue;
                }
                self.mark_reachable(e.node2, out, filter);
            }
        }

        pub fn mark_reachable_any(&self, s: usize, out: &mut [bool]) {
            self.mark_reachable(s, out, &|_| true);
        }

        pub fn mark_reachable_capable(&self, s: usize, out: &mut [bool]) {
            self.mark_reachable(s, out, &|e| e.capacity != 0 && e.is_real);
        }
    }
}
