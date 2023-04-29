use std::collections::VecDeque;
use std::mem::swap;

fn main() {
    let mut lines = std::io::stdin().lines().map(|s| {
        s.unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>()
    });
    let nmst = lines.next().unwrap();
    let n = nmst[0];
    let m = nmst[1];
    let s = nmst[2];
    let t = nmst[3];
    let mut edges = Vec::with_capacity(m);
    for _ in 0..m {
        let xy = lines.next().unwrap();
        edges.push([xy[0], xy[1]]);
    }
    let answer = solve(n, s, t, &edges);
    if let Some([path1, path2]) = answer {
        println!("YES");
        for x in path1 {
            print!("{x} ");
        }
        for x in path2 {
            print!("{x} ");
        }
        println!();
    } else {
        println!("NO");
    }
}

fn solve(n: usize, s: usize, t: usize, edges_inp: &[[usize; 2]]) -> Option<[Vec<usize>; 2]> {
    let s = s - 1;
    let t = t - 1;
    let mut g = Graph::new(n, edges_inp);
    let el1 = g.subflow(s, t)?;
    let el2 = g.subflow(s, t)?;
    debug!(&el1; &el2);
    let mut path1 = vec![s + 1];
    let mut path2 = vec![s + 1];
    let mut pos1 = s;
    let mut pos2 = s;
    let mut i = 0;
    let mut j = 0;
    while j < el2.len() {
        let e2 = g.edges[pos2][el2[j]];
        if e2.is_fwd {
            let next = e2.node;
            path2.push(next + 1);
            pos2 = next;
        } else {
            loop {
                if pos1 == e2.node && el1[i] == e2.rev_pos {
                    swap(&mut pos1, &mut pos2);
                    swap(&mut path1, &mut path2);
                    i += 1;
                    break;
                } else {
                    let e1 = g.edges[pos1][el1[i]];
                    let next = e1.node;
                    path1.push(next + 1);
                    pos1 = next;
                    i += 1;
                }
            }
        }
        j += 1;
    }
    while i < el1.len() {
        let next = g.edges[pos1][el1[i]].node;
        path1.push(next + 1);
        pos1 = next;
        i += 1;
    }
    if path1 >= path2 {
        Some([path1, path2])
    } else {
        Some([path2, path1])
    }
}

#[derive(Debug, Clone)]
struct Graph {
    n_nodes: usize,
    n_edges: usize,
    edges: Vec<Vec<Edge>>,
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    node: usize,
    rev_pos: usize,
    capacity: i32,
    is_fwd: bool,
}

impl Graph {
    fn new(n: usize, edges_inp: &[[usize; 2]]) -> Self {
        let mut edges = vec![vec![]; n];
        for &[u, v] in edges_inp {
            let u = u - 1;
            let v = v - 1;
            let i = edges[u].len();
            let j = edges[v].len();
            edges[u].push(Edge {
                node: v,
                rev_pos: j,
                capacity: 1,
                is_fwd: true,
            });
            edges[v].push(Edge {
                node: u,
                rev_pos: i,
                capacity: 0,
                is_fwd: false,
            });
        }
        Self {
            n_nodes: n,
            n_edges: edges_inp.len(),
            edges,
        }
    }

    /// returns edge list
    fn subflow(&mut self, s: usize, t: usize) -> Option<Vec<usize>> {
        if s == t {
            return Some(vec![]);
        }
        let mut back_edge = vec![usize::MAX; self.n_nodes];
        let mut queue = VecDeque::new();
        queue.push_back(s);
        while !queue.is_empty() {
            let u = queue.pop_front().unwrap();
            for ei in 0..self.edges[u].len() {
                let e = self.edges[u][ei];
                let v = e.node;
                if e.capacity == 0 {
                    continue;
                }
                if back_edge[v] != usize::MAX {
                    continue;
                }
                queue.push_back(v);
                back_edge[v] = e.rev_pos;
            }
        }
        if back_edge[t] == usize::MAX {
            return None;
        }
        let mut result = vec![];
        let mut node = t;
        while node != s {
            let e = self.edges[node][back_edge[node]];
            let cap = self.edges[e.node][e.rev_pos].capacity;
            self.edges[node][back_edge[node]].capacity += cap;
            self.edges[e.node][e.rev_pos].capacity -= cap;
            result.push(e.rev_pos);
            node = e.node;
        }
        result.reverse();
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // 3 3 1 3
        // 1 2
        // 1 3
        // 2 3
        // ========
        // YES
        // 1 3 1 2 3
        let edges = vec![[1, 2], [1, 3], [2, 3]];
        let actual = solve(3, 1, 3, &edges);
        let expected = Some([vec![1, 3], vec![1, 2, 3]]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test2() {
        // 4 5 1 4
        // 1 2
        // 1 3
        // 2 3
        // 2 4
        // 3 4
        // ========
        // YES
        // 1 3 4 1 2 4
        let edges = vec![[1, 2], [1, 3], [2, 3], [2, 4], [3, 4]];
        let actual = solve(4, 1, 4, &edges);
        let expected = Some([vec![1, 3, 4], vec![1, 2, 4]]);
        assert_eq!(expected, actual);
    }
}

#[allow(unused)]
mod util {
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
}
