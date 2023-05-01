use util::{debug, Graph};

fn main() {
    let mut lines = std::io::stdin().lines().map(|s| s.unwrap());
    let nm_str = lines.next().unwrap();
    let (n_str, _) = nm_str.trim().split_once(' ').unwrap();
    let n = n_str.parse().unwrap();
    let strings: Vec<String> = lines.take(n).collect();
    let trimmed: Vec<&str> = strings.iter().map(|s| s.trim()).collect();
    let answer = solve(&trimmed);
    if answer {
        println!("Valid");
    } else {
        println!("Invalid");
    }
}

fn solve(lines: &[&str]) -> bool {
    let n = lines.len();
    debug_assert_ne!(0, n);
    let m = lines[0].len();
    #[cfg(test)]
    for i in 0..n {
        debug_assert_eq!(m, lines[i].len());
    }

    let mut total_cap = 0;
    let mut g = Graph::new(2 * n * m + 2);
    let s = 2 * n * m;
    let t = s + 1;
    for i in 0..n {
        for (j, c) in lines[i].chars().enumerate() {
            let wrap = i * m + j;
            let cap = match c {
                '.' => 0,
                'H' => 1,
                'O' => 2,
                'N' => 3,
                'C' => 4,
                _ => panic!("Unexpected char {c}"),
            };
            total_cap += cap;
            g.add_edge(2 * wrap, 2 * wrap + 1, cap);

            if (i + j) % 2 == 0 {
                g.add_edge(s, 2 * wrap, cap);
            } else {
                g.add_edge(2 * wrap + 1, t, cap);
            }
        }
    }

    if total_cap == 0 {
        return false;
    }

    for i in 0..n {
        for j in 1..m {
            let wrap1 = i * m + j - 1;
            let wrap2 = i * m + j;
            if (i + j) % 2 == 0 {
                g.add_edge(2 * wrap2 + 1, 2 * wrap1, 1);
            } else {
                g.add_edge(2 * wrap1 + 1, 2 * wrap2, 1);
            }
        }
    }

    for i in 1..n {
        for j in 0..m {
            let wrap1 = (i - 1) * m + j;
            let wrap2 = i * m + j;
            if (i + j) % 2 == 0 {
                g.add_edge(2 * wrap2 + 1, 2 * wrap1, 1);
            } else {
                g.add_edge(2 * wrap1 + 1, 2 * wrap2, 1);
            }
        }
    }

    let mut achieved = 0;
    while {
        let flow = g.mark_subflow(s, t);
        achieved += flow;
        flow != 0
    } {}

    debug!(achieved, total_cap);
    2 * achieved == total_cap
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // 3 4
        // HOH.
        // NCOH
        // OO..
        // ========
        // Valid
        let answer = solve(&["HOH.", "NCOH", "OO.."]);
        assert!(answer);
    }

    #[test]
    fn test2() {
        // 3 4
        // HOH.
        // NCOH
        // OONH
        // ========
        // Invalid
        let answer = solve(&["HOH.", "NCOH", "OONH"]);
        assert!(!answer);
    }
}
