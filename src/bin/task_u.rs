use crate::util::calc_zfun;

fn main() {
    let line = std::io::stdin().lines().next().unwrap().unwrap();
    let answer = solve(line.trim());
    println!("{answer}");
}

fn solve(input: &str) -> usize {
    let s: Vec<char> = input.chars().collect();
    let n = s.len();
    let mut zfun = vec![0; n];
    calc_zfun(&s, &mut zfun);
    for i in 0..n {
        if i + zfun[i] == n {
            return i;
        }
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let actual = solve("zzz");
        assert_eq!(1, actual);
    }

    #[test]
    fn test2() {
        let actual = solve("bcabcab");
        assert_eq!(3, actual);
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
