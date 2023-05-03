use util::{Capacity::*, Graph};

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
            let pos = g.add_edge(v, n + u - 1, Finite(1));
            init_edges.push(pos);
        }
    }

    for v in 0..n {
        let u = n + m + v;
        g.add_edge(s, u, Finite(1));
        g.add_edge(u, v, Finite(1));
    }

    for v in 0..m {
        let v = n + v;
        let u = n + m + v;
        g.add_edge(v, u, Finite(1));
        g.add_edge(u, t, Finite(1));
    }

    g.max_flow(s, t);

    let mut answer = Vec::new();
    for i in init_edges {
        let e = g.edge(i);
        if e.capacity != Finite(0) {
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
