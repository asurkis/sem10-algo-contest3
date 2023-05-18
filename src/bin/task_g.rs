use util::{Capacity::*, Graph};

fn main() {
    let mut lines = std::io::stdin().lines().map(|s| {
        s.unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i64>>()
    });
    let n = lines.next().unwrap()[0] as usize;
    let usefulness = lines.next().unwrap();
    let dependencies: Vec<Vec<usize>> = lines
        .take(n)
        .map(|v| v.iter().skip(1).map(|&x| x as usize).collect())
        .collect();
    let answer = solve(&usefulness, &dependencies);
    println!("{answer}");
}

fn solve(usefulness: &[i64], dependencies: &[Vec<usize>]) -> u64 {
    let n = usefulness.len();
    debug_assert_eq!(n, dependencies.len());
    let mut graph = Graph::new(n + 2);
    let s = n;
    let t = s + 1;
    for i in 0..n {
        if usefulness[i] >= 0 {
            graph.add_edge(s, i, Finite(usefulness[i] as u64));
        } else {
            graph.add_edge(i, t, Finite((-usefulness[i]) as u64));
        }
    }
    for i in 0..n {
        for j in &dependencies[i] {
            let j = *j - 1;
            graph.add_edge(i, j, Infinity);
        }
    }

    let _ = graph.max_flow(s, t);
    let reachable = graph.find_reachable_capable(s);
    let mut result = 0;
    for i in 0..n {
        if reachable[i] {
            result += usefulness[i];
        }
    }
    result as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // 4
        // -1 1 -2 2
        // 0
        // 1 1
        // 2 4 2
        // 1 1
        // ========
        // 2
        let answer = solve(&[-1, 1, -2, 2], &[vec![], vec![1], vec![4, 2], vec![1]]);
        assert_eq!(2, answer);
    }

    #[test]
    fn test2() {
        // 3
        // 2 -1 -2
        // 2 2 3
        // 0
        // 0
        // ========
        // 0
        let answer = solve(&[2, -1, -2], &[vec![2, 3], vec![], vec![]]);
        assert_eq!(0, answer);
    }
}
