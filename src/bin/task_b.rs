use util::Graph;

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
    let mut edges = Vec::with_capacity(m);
    for _ in 0..m {
        let mut v = lines.next().unwrap();
        v.remove(0);
        edges.push(v);
    }
    let max_pair = lines.next().unwrap();
    let (answer1, answer2) = solve(m, n, &edges, &max_pair);
    println!("{}", answer1.len() + answer2.len());
    print!("{}", answer1.len());
    for i in answer1 {
        print!(" {i}");
    }
    println!();
    print!("{}", answer2.len());
    for i in answer2 {
        print!(" {i}");
    }
    println!();
}

fn solve(m: usize, n: usize, edges: &[Vec<usize>], max_pair: &[usize]) -> (Vec<usize>, Vec<usize>) {
    assert_eq!(m, edges.len());
    let mut graph = Graph::new(m + n);
    for i in 0..m {
        if max_pair[i] != 0 {
            graph.add_edge(m + max_pair[i] - 1, i, 1);
        }
        for &j in &edges[i] {
            if j != max_pair[i] {
                graph.add_edge(i, m + j - 1, 1);
            }
        }
    }

    let mut reachable = vec![false; m + n];
    for i in 0..m {
        if max_pair[i] == 0 {
            graph.mark_reachable_capable(i, &mut reachable);
        }
    }

    let mut answer1 = Vec::new();
    let mut answer2 = Vec::new();
    for i in 0..m {
        if !reachable[i] {
            answer1.push(i + 1);
        }
    }
    for i in 0..n {
        if reachable[m + i] {
            answer2.push(i + 1);
        }
    }
    (answer1, answer2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // 1 1
        // 1 1
        // 1
        // ========
        // 1
        // 1 1
        // 0
        let answer = solve(1, 1, &[vec![1]], &[1]);
        assert_eq!((vec![1], vec![]), answer);
    }

    #[test]
    fn test2() {
        // 3 2
        // 2 1 2
        // 1 2
        // 1 2
        // 1 2 0
        // ========
        // 2
        // 1 1
        // 1 2
        let answer = solve(3, 2, &[vec![1, 2], vec![2], vec![2]], &[1, 2, 0]);
        assert_eq!((vec![1], vec![2]), answer);
    }
}
