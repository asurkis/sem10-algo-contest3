use util::*;

fn main() {
    let mut lines = std::io::stdin().lines().map(|s| {
        s.unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>()
    });
    let mn = lines.next().unwrap();
    let m = mn[0];
    let n = mn[1];
    let kl = lines.next().unwrap();
    let k = kl[0];
    let l = kl[1];
    let mut mountains = Vec::with_capacity(k);
    for _ in 0..k {
        let xy = lines.next().unwrap();
        mountains.push([xy[0], xy[1]]);
    }
    let mut wall_options = Vec::with_capacity(l);
    for _ in 0..l {
        let xy = lines.next().unwrap();
        wall_options.push([xy[0], xy[1]]);
    }
    let a = lines.next().unwrap();
    let b = lines.next().unwrap();
    let answer = solve(
        [m, n],
        [a[0], a[1]],
        [b[0], b[1]],
        &mountains,
        &wall_options,
    );
    println!("{answer}");
}

fn solve(
    [m, n]: [usize; 2],
    [xa, ya]: [usize; 2],
    [xb, yb]: [usize; 2],
    mountains: &[[usize; 2]],
    wall_options: &[[usize; 2]],
) -> i64 {
    // wrap = x + m * y
    // v_in = 2 * wrap
    // v_out = v_in + 1
    let mut is_mountain = vec![false; n * m];
    let mut can_wall = vec![false; n * m];
    for &[x, y] in mountains {
        is_mountain[(x - 1) + m * (y - 1)] = true;
    }
    for &[x, y] in wall_options {
        can_wall[(x - 1) + m * (y - 1)] = true;
    }
    let mut g = Graph::new(2 * n * m);
    for y in 0..n {
        for x in 0..m {
            let wrap = x + m * y;
            let capacity = if can_wall[wrap] { 1 } else { Capacity::MAX / 2 };
            g.add_edge(2 * wrap, 2 * wrap + 1, capacity);
        }
    }
    for y in 0..n {
        for x in 1..m {
            let wrap1 = (x - 1) + m * y;
            let wrap2 = x + m * y;
            if !is_mountain[wrap1] && !is_mountain[wrap2] {
                g.add_biedge(2 * wrap1 + 1, 2 * wrap2, 1);
            }
        }
    }
    for y in 1..n {
        for x in 0..m {
            let wrap1 = x + m * (y - 1);
            let wrap2 = x + m * y;
            if !is_mountain[wrap1] && !is_mountain[wrap2] {
                g.add_biedge(2 * wrap1 + 1, 2 * wrap2, Capacity::MAX / 2);
            }
        }
    }

    let wrap_s = 2 * ((xa - 1) + m * (ya - 1));
    let wrap_t = 2 * ((xb - 1) + m * (yb - 1)) + 1;

    let mut answer = 0;
    while g.mark_subflow(wrap_s, wrap_t) != 0 {
        answer += 1;
    }

    let mut reachable = vec![false; g.n_nodes()];
    debug_assert!(!reachable[wrap_t]);
    g.mark_reachable_capable(wrap_s, &mut reachable);
    debug!(&reachable);
    for ei in 0..g.n_edges() {
        let e = g.edge(ei);
        debug!(e, reachable[e.node1], reachable[e.node2]);
        if reachable[e.node1] != reachable[e.node2] && e.capacity > 2 {
            return -1;
        }
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // 1 2
        // 0 0
        // 1 1
        // 1 2
        // ========
        // -1
        let mut actual = solve([1, 2], [1, 1], [1, 2], &[], &[]);
        assert_eq!(-1, actual);
    }

    #[test]
    fn test2() {
        // 2 2
        // 2 0
        // 1 2
        // 2 1
        // 1 1
        // 2 2
        // ========
        // 0
        let mut actual = solve([2, 2], [1, 1], [2, 2], &[[1, 2], [2, 1]], &[]);
        assert_eq!(0, actual);
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

    pub type Capacity = u32;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Edge {
        pub node1: usize,
        pub node2: usize,
        pub capacity: Capacity,
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
            self.add_one_edge(node2, node1, 0);
            self.add_one_edge(node1, node2, capacity)
        }

        pub fn add_biedge(&mut self, node1: usize, node2: usize, capacity: Capacity) -> usize {
            self.add_one_edge(node2, node1, capacity);
            self.add_one_edge(node1, node2, capacity)
        }

        fn add_one_edge(&mut self, node1: usize, node2: usize, capacity: Capacity) -> usize {
            let pos = self.n_edges();
            self.edge_idx[node1].push(pos);
            self.edge_heap.push(Edge {
                node1,
                node2,
                capacity,
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
            self.mark_reachable(s, out, &|e| e.capacity != 0);
        }
    }
}
