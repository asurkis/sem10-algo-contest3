use std::{
    cmp::Ordering,
    collections::VecDeque,
    ops::{Add, AddAssign, Sub, SubAssign},
};

// #[cfg(test)]
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

/*
#[cfg(not(test))]
#[macro_export]
macro_rules! debug {
    ($($($val:expr),+);*) => {};
}
*/

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

pub fn calc_zfun_inplace(s: &[impl Eq], z: &mut [usize]) {
    let n = s.len();
    assert_eq!(n, z.len());
    if n == 0 {
        return;
    }
    z[0] = n;
    if n == 1 {
        return;
    }
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

pub fn calc_zfun(s: &[impl Eq]) -> Vec<usize> {
    let mut zfun = vec![0; s.len()];
    calc_zfun_inplace(s, &mut zfun);
    zfun
}

pub fn calc_pfun_inplace(s: &[impl Eq], p: &mut [usize]) {
    let n = s.len();
    assert_eq!(n, p.len());
    if n == 0 {
        return;
    }
    p[0] = 0;
    for i in 1..n {
        let mut k = p[i - 1];
        while k != 0 && s[i] != s[k] {
            k = p[k - 1];
        }
        if s[i] == s[k] {
            k += 1;
        }
        p[i] = k;
    }
}

pub fn calc_pfun(s: &[impl Eq]) -> Vec<usize> {
    let mut pfun = vec![0; s.len()];
    calc_pfun_inplace(s, &mut pfun);
    pfun
}

use Capacity::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord)]
pub enum Capacity {
    Finite(u64),
    Infinity,
}

impl Capacity {
    pub fn unwrap(self) -> u64 {
        match self {
            Finite(x) => x,
            Infinity => panic!(),
        }
    }
}

impl PartialOrd for Capacity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Finite(x) => match other {
                Finite(y) => x.partial_cmp(y),
                Infinity => Some(Ordering::Less),
            },
            Infinity => match other {
                Finite(_) => Some(Ordering::Greater),
                Infinity => Some(Ordering::Equal),
            },
        }
    }
}

impl Add for Capacity {
    type Output = Capacity;
    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Finite(x) => match rhs {
                Finite(y) => Finite(x + y),
                Infinity => Infinity,
            },
            Infinity => Infinity,
        }
    }
}

impl Sub for Capacity {
    type Output = Capacity;
    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Finite(x) => match rhs {
                Finite(y) => Finite(x - y),
                Infinity => unreachable!(),
            },
            Infinity => match rhs {
                Finite(_) => Infinity,
                Infinity => Infinity,
            },
        }
    }
}

impl AddAssign for Capacity {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for Capacity {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

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
        self.add_one_edge(node2, node1, Finite(0), false);
        self.add_one_edge(node1, node2, capacity, true)
    }

    pub fn add_biedge(&mut self, node1: usize, node2: usize, capacity: Capacity) -> usize {
        self.add_one_edge(node2, node1, capacity, false);
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
                if e.capacity == Finite(0) {
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
            return Finite(0);
        }

        let mut flow = Infinity;
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

    pub fn max_flow(&mut self, s: usize, t: usize) -> Capacity {
        let mut flow = Finite(0);
        while {
            let sub = self.mark_subflow(s, t);
            flow += sub;
            match sub {
                Finite(0) => false,
                Finite(_) => true,
                Infinity => false,
            }
        } {}
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
        self.mark_reachable(s, out, &|e| e.capacity != Finite(0) && e.is_real);
    }

    pub fn find_reachable(&self, s: usize, filter: &impl Fn(&Edge) -> bool) -> Vec<bool> {
        let mut out = vec![false; self.n_nodes()];
        self.mark_reachable(s, &mut out, filter);
        out
    }

    pub fn find_reachable_any(&self, s: usize) -> Vec<bool> {
        self.find_reachable(s, &|_| true)
    }

    pub fn find_reachable_capable(&self, s: usize) -> Vec<bool> {
        self.find_reachable(s, &|e| e.capacity != Finite(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pfun() {
        let chars: Vec<char> = "abcabcd".chars().collect();
        let pfun1 = calc_pfun(&chars);
        assert_eq!(vec![0, 0, 0, 1, 2, 3, 0], pfun1);
    }
}
