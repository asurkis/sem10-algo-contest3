use util::{Capacity::*, Graph};

fn main() {
    let mut lines = std::io::stdin().lines().map(|s| s.unwrap());
    let first_line: Vec<u64> = lines
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let n = first_line[0] as usize;
    let m = first_line[1] as usize;
    let w = first_line[2] as u64;
    let b = first_line[3] as u64;
    let g = first_line[4] as u64;
    let painting: Vec<String> = lines.take(n).collect();
    let painting_str: Vec<&str> = painting.iter().map(|s| s.trim()).collect();
    let answer = solve([n, m], [w, b, g], &painting_str);
    println!("{answer}");
}

fn solve([n, m]: [usize; 2], [w, b, g]: [u64; 3], painting: &[&str]) -> u64 {
    debug_assert_eq!(n, painting.len());
    for i in 0..n {
        debug_assert_eq!(m, painting[i].len());
    }

    let mut graph = Graph::new(n * m + 2);
    let s = graph.n_nodes() - 2;
    let t = s + 1;

    for i in 0..n {
        for (j, c) in painting[i].chars().enumerate() {
            let wrap = i * m + j;
            match c {
                'B' => graph.add_edge(s, wrap, Finite(w)),
                'W' => graph.add_edge(wrap, t, Finite(b)),
                _ => panic!("Unknown color {c}"),
            };
        }
    }

    for i in 0..n {
        for j in 1..m {
            let wrap1 = i * m + j - 1;
            let wrap2 = i * m + j;
            graph.add_biedge(wrap1, wrap2, Finite(g));
        }
    }

    for i in 1..n {
        for j in 0..m {
            let wrap1 = (i - 1) * m + j;
            let wrap2 = i * m + j;
            graph.add_biedge(wrap1, wrap2, Finite(g));
        }
    }

    graph.max_flow(s, t).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // 3 2 10 12 1
        // BW
        // WB
        // BW
        // ========
        // 7
        let answer = solve([3, 2], [10, 12, 1], &["BW", "WB", "BW"]);
        assert_eq!(7, answer);
    }

    #[test]
    fn test2() {
        // 10 1 2 3 5
        // W
        // W
        // W
        // B
        // B
        // B
        // B
        // W
        // W
        // B
        // ========
        // 10
        let answer = solve(
            [10, 1],
            [2, 3, 5],
            &["W", "W", "W", "B", "B", "B", "B", "W", "W", "B"],
        );
        assert_eq!(10, answer);
    }

    #[test]
    fn test_my1() {
        // 1 10 2 3 5
        // WWWBBBBWWB
        // ========
        // 10
        let answer = solve([1, 10], [2, 3, 5], &["WWWBBBBWWB"]);
        assert_eq!(10, answer);
    }

    #[test]
    fn test_my2() {
        // 1 10 2 3 5
        // WWWBBBBWWB
        // ========
        // 10
        let answer = solve([1, 2], [2, 3, 5], &["WB"]);
        assert_eq!(2, answer);
    }
}
