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
    if [xa, ya] == [xb, yb] {
        return -1;
    }
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
            let capacity = if can_wall[wrap] { 1 } else { Capacity::MAX };
            g.add_edge(2 * wrap, 2 * wrap + 1, capacity);
        }
    }
    for y in 0..n {
        for x in 1..m {
            let wrap1 = (x - 1) + m * y;
            let wrap2 = x + m * y;
            if !is_mountain[wrap1] && !is_mountain[wrap2] {
                g.add_edge(2 * wrap1 + 1, 2 * wrap2, Capacity::MAX);
                g.add_edge(2 * wrap2 + 1, 2 * wrap1, Capacity::MAX);
            }
        }
    }
    for y in 1..n {
        for x in 0..m {
            let wrap1 = x + m * (y - 1);
            let wrap2 = x + m * y;
            if !is_mountain[wrap1] && !is_mountain[wrap2] {
                g.add_edge(2 * wrap1 + 1, 2 * wrap2, Capacity::MAX);
                g.add_edge(2 * wrap2 + 1, 2 * wrap1, Capacity::MAX);
            }
        }
    }

    let wrap_s = 2 * ((xa - 1) + m * (ya - 1));
    let wrap_t = 2 * ((xb - 1) + m * (yb - 1)) + 1;
    debug!("Before subflows");
    for ei in 0..g.n_edges() {
        debug!(ei, g.edge(ei));
    }

    let mut answer = 0;
    while g.mark_subflow(wrap_s, wrap_t) != 0 {
        answer += 1;
    }
    debug!("After subflows");
    for ei in 0..g.n_edges() {
        debug!(ei, g.edge(ei));
    }

    let mut reachable = vec![false; g.n_nodes()];
    debug_assert!(!reachable[wrap_t]);
    g.mark_reachable_capable(wrap_s, &mut reachable);
    debug!(&reachable);
    for ei in 0..g.n_edges() {
        let e1 = g.edge(ei);
        let e2 = g.edge(ei ^ 1);
        debug_assert_eq!(e1.node1, e2.node2);
        debug_assert_eq!(e1.node2, e2.node1);
        if e1.is_real
            && reachable[e1.node1]
            && !reachable[e1.node2]
            && e1.capacity + e2.capacity == Capacity::MAX
        {
            return -1;
        }
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test1() {
        // 1 2
        // 0 0
        // 1 1
        // 1 2
        // ========
        // -1
        let actual = solve([1, 2], [1, 1], [1, 2], &[], &[]);
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
        let actual = solve([2, 2], [1, 1], [2, 2], &[[1, 2], [2, 1]], &[]);
        assert_eq!(0, actual);
    }

    #[test]
    fn test_my1() {
        // a.b
        let actual = solve([3, 1], [1, 1], [3, 1], &[], &[[2, 1]]);
        assert_eq!(1, actual);
    }

    fn gen_input(
        [max_m, max_n]: [usize; 2],
    ) -> impl Strategy<
        Value = (
            [usize; 2],
            [usize; 2],
            [usize; 2],
            Vec<[usize; 2]>,
            Vec<[usize; 2]>,
        ),
    > {
        [1..max_m, 1..max_n].prop_flat_map(move |[m, n]| {
            ([1..=m, 1..=n], [1..=m, 1..=n], vec![0..2; m * n]).prop_map(
                move |([xa, ya], [xb, yb], vec)| {
                    let mut mountains = Vec::new();
                    let mut wall_options = Vec::new();
                    for y in 0..n {
                        for x in 0..m {
                            if [x + 1, y + 1] == [xa, ya] || [x + 1, y + 1] == [xb, yb] {
                                continue;
                            }
                            let wrap = x + m * y;
                            if vec[wrap] == 1 {
                                mountains.push([x + 1, y + 1]);
                            } else if vec[wrap] == 2 {
                                wall_options.push([x + 1, y + 1]);
                            }
                        }
                    }
                    ([m, n], [xa, ya], [xb, yb], mountains, wall_options)
                },
            )
        })
    }

    fn check_passability(
        [m, n]: [usize; 2],
        [xa, ya]: [usize; 2],
        [xb, yb]: [usize; 2],
        mountains: &[[usize; 2]],
        wall_options: &[[usize; 2]],
    ) -> i64 {
        if [xa, ya] == [xb, yb] {
            return -1;
        }
        let wrap_s = (xa - 1) + m * (ya - 1);
        let wrap_t = (xb - 1) + m * (yb - 1);
        let mut is_passable = vec![true; m * n];
        for &[x, y] in mountains {
            let wrap = (x - 1) + m * (y - 1);
            is_passable[wrap] = false;
        }
        for &[x, y] in wall_options {
            let wrap = (x - 1) + m * (y - 1);
            is_passable[wrap] = false;
        }
        assert!(is_passable[wrap_s]);
        assert!(is_passable[wrap_t]);
        let mut g = Graph::new(m * n);
        for y in 0..n {
            for x in 1..m {
                let wrap1 = (x - 1) + m * y;
                let wrap2 = x + m * y;
                if is_passable[wrap1] && is_passable[wrap2] {
                    g.add_edge(wrap1, wrap2, 1);
                }
            }
        }
        for y in 1..n {
            for x in 0..m {
                let wrap1 = x + m * (y - 1);
                let wrap2 = x + m * y;
                if is_passable[wrap1] && is_passable[wrap2] {
                    g.add_edge(wrap1, wrap2, 1);
                }
            }
        }

        let mut reachable = vec![false; m * n];
        g.mark_reachable_any(wrap_s, &mut reachable);
        if reachable[wrap_t] {
            -1
        } else {
            wall_options.len() as i64
        }
    }

    fn compare_with_baseline(
        (mn, a, b, mountains, wall_options): (
            [usize; 2],
            [usize; 2],
            [usize; 2],
            Vec<[usize; 2]>,
            Vec<[usize; 2]>,
        ),
    ) {
        let expected = check_passability(mn, a, b, &mountains, &wall_options);
        let actual = solve(mn, a, b, &mountains, &wall_options);
        if expected == -1 {
            assert_eq!(-1, actual);
        } else {
            assert_ne!(-1, actual);
            assert!(0 <= actual);
            assert!(actual <= expected);
        }
    }

    proptest! {
        #[test]
        fn test_props(input in gen_input([10, 10])) {
            compare_with_baseline(input);
        }
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
