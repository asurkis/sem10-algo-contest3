use util::{Capacity::*, Graph};

fn main() {}

fn solve(n: usize, pairs: &[[usize; 2]]) -> Vec<usize> {
    let mut graph = Graph::new(n + 2);
    let s = 0;
    let t = n + 1;
    for v in 1..=n {
        graph.add_edge(s, v, Finite(1));
        graph.add_edge(v, t, Finite(2));
    }
    for &[u, v] in pairs {
        graph.add_biedge(u, v, Finite(1));
    }
    let max_flow = graph.max_flow(s, t).unwrap() as usize;
    let reachable = graph.find_reachable_capable(s);
    let mut answer = Vec::with_capacity(max_flow);
    for v in 1..=n {
        if reachable[v] {
            answer.push(v);
        }
    }
    debug_assert_eq!(max_flow, answer.len());
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // 5 6
        // 1 5
        // 5 4
        // 4 2
        // 2 5
        // 1 2
        // 3 1
        // ========
        // 4
        // 1
        // 2
        // 4
        // 5
        let actual = solve(5, &[[1, 5], [5, 4], [4, 2], [2, 5], [1, 2], [3, 1]]);
        let expected = vec![1, 2, 4, 5];
        assert_eq!(expected, actual);
    }

    #[test]
    fn test2() {
        // 4 0
        // ========
        // 1
        // 1
        let actual = solve(4, &[]);
        let expected = vec![1];
        assert_eq!(expected, actual);
    }
}
