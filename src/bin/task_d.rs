use crate::util::Graph;

fn main() {
    let mut lines = std::io::stdin().lines().map(|s| s.unwrap());
    let n: usize = lines.next().unwrap().trim().parse().unwrap();
    let mut orders = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap();
        let split: Vec<&str> = line.trim().split_whitespace().collect();
        let (hour_str, minute_str) = split[0].split_once(':').unwrap();
        let hour = hour_str.parse().unwrap();
        let minute = minute_str.parse().unwrap();
        let sx = split[1].parse().unwrap();
        let sy = split[2].parse().unwrap();
        let tx = split[3].parse().unwrap();
        let ty = split[4].parse().unwrap();
        let order = Order::new(hour, minute, sx, sy, tx, ty);
        orders.push(order);
    }

    let answer = solve(&orders);
    println!("{answer}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Order {
    // u16 достаточно на самом деле
    epoch: u64,
    sx: u64,
    sy: u64,
    tx: u64,
    ty: u64,
}

impl Order {
    fn new(hours: u64, minutes: u64, sx: u64, sy: u64, tx: u64, ty: u64) -> Self {
        Self {
            epoch: 60 * hours + minutes,
            sx,
            sy,
            tx,
            ty,
        }
    }

    fn precedes(self, that: Order) -> bool {
        self.epoch
            + self.sx.abs_diff(self.tx)
            + self.sy.abs_diff(self.ty)
            + self.tx.abs_diff(that.sx)
            + self.ty.abs_diff(that.sy)
            < that.epoch
    }
}

fn solve(orders: &[Order]) -> usize {
    let n = orders.len();
    let mut h = Graph::new(2 * n + 2);
    let s = 2 * n;
    let t = s + 1;
    let mut og_edges = Vec::new();
    for i in 0..n {
        for j in 0..n {
            if orders[i].precedes(orders[j]) {
                let pos = h.add_edge(i, n + j);
                og_edges.push(pos);
            }
        }
    }
    for i in 0..n {
        h.add_edge(s, i);
        // g.add_edge(n + i, i);
        h.add_edge(n + i, t);
    }

    while h.mark_subflow(s, t) {}

    let mut g = Graph::new(n);
    for ei in og_edges {
        let e = h.get_edge(ei);
        g.add_edge(e.node1, e.node2 - n);
    }

    let mut reachable = vec![false; n];
    let mut answer = 0;
    for i in 0..n {
        if reachable[i] {
            continue;
        }
        g.mark_reachable(i, &mut reachable);
        answer += 1;
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // 2
        // 08:00 10 11 9 16
        // 08:07 9 16 10 11
        // ========
        // 1
        let actual = solve(&vec![
            Order::new(8, 0, 10, 11, 9, 16),
            Order::new(8, 7, 9, 16, 10, 11),
        ]);
        assert_eq!(1, actual);
    }

    #[test]
    fn test2() {
        // 2
        // 08:00 10 11 9 16
        // 08:06 9 16 10 11
        // ========
        // 2
        let actual = solve(&vec![
            Order::new(8, 0, 10, 11, 9, 16),
            Order::new(8, 6, 9, 16, 10, 11),
        ]);
        assert_eq!(2, actual);
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

        pub fn mark_reachable(&self, s: usize, out: &mut [bool]) {
            if out[s] {
                return;
            }
            out[s] = true;
            for &ei in &self.edge_idx[s] {
                self.mark_reachable(self.edge_heap[ei].node2, out);
            }
        }
    }
}
