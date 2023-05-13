use util::{Capacity::*, Graph};

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
    match answer {
        None => println!("-1"),
        Some(answer) => {
            println!("{}", answer.len());
            for [x, y] in answer {
                println!("{x} {y}");
            }
        }
    }
}

fn solve(
    [m, n]: [usize; 2],
    [xa, ya]: [usize; 2],
    [xb, yb]: [usize; 2],
    mountains: &[[usize; 2]],
    wall_options: &[[usize; 2]],
) -> Option<Vec<[usize; 2]>> {
    if [xa, ya] == [xb, yb] {
        return None;
    }
    // wrap = x + m * y
    // v_in = 2 * wrap
    // v_out = v_in + 1
    let mut can_wall = vec![false; n * m];
    let mut is_mountain = vec![false; n * m];
    for &[x, y] in wall_options {
        can_wall[(x - 1) + m * (y - 1)] = true;
    }
    for &[x, y] in mountains {
        is_mountain[(x - 1) + m * (y - 1)] = true;
    }

    #[cfg(test)]
    for y in 0..n {
        for x in 0..m {
            let wrap = x + m * y;
            if is_mountain[wrap] {
                print!("m");
            } else if can_wall[wrap] {
                print!("w");
            } else if [x + 1, y + 1] == [xa, ya] {
                print!("s");
            } else if [x + 1, y + 1] == [xb, yb] {
                print!("t");
            } else {
                print!(".");
            }
        }
        println!();
    }
    #[cfg(test)]
    println!();

    let mut graph = Graph::new(2 * m * n);
    let wrap_s = (xa - 1) + m * (ya - 1);
    let wrap_t = (xb - 1) + m * (yb - 1);

    for y in 0..n {
        for x in 0..m {
            let wrap = x + m * y;
            let cap = if is_mountain[wrap] {
                Finite(0)
            } else if can_wall[wrap] {
                Finite(1)
            } else {
                Infinite
            };
            graph.add_edge(2 * wrap, 2 * wrap + 1, cap);
        }
    }

    for y in 0..n {
        for x in 0..m {
            let wrap1 = x + m * y;
            let wrap2 = wrap1 + 1;
            let wrap3 = wrap1 + m;
            if x + 1 < m {
                graph.add_edge(2 * wrap1 + 1, 2 * wrap2, Infinite);
                graph.add_edge(2 * wrap2 + 1, 2 * wrap1, Infinite);
            }
            if y + 1 < n {
                graph.add_edge(2 * wrap1 + 1, 2 * wrap3, Infinite);
                graph.add_edge(2 * wrap3 + 1, 2 * wrap1, Infinite);
            }
        }
    }

    let flow = graph.max_flow(2 * wrap_s, 2 * wrap_t + 1);
    match flow {
        Finite(flow) => {
            let reachable = graph.find_reachable_capable(2 * wrap_s);
            let mut is_wall = vec![false; n * m];
            for y in 0..n {
                for x in 0..m {
                    let wrap = x + m * y;
                    if !can_wall[wrap] || !reachable[2 * wrap] {
                        continue;
                    }

                    is_wall[wrap] = false
                        || (x > 0 && !is_mountain[wrap - 1] && !reachable[2 * (wrap - 1)])
                        || (y > 0 && !is_mountain[wrap - m] && !reachable[2 * (wrap - m)])
                        || (x + 1 < m && !is_mountain[wrap + 1] && !reachable[2 * (wrap + 1)])
                        || (y + 1 < n && !is_mountain[wrap + m] && !reachable[2 * (wrap + m)]);
                }
            }
            let mut answer = Vec::with_capacity(flow as usize);
            for y in 0..n {
                for x in 0..m {
                    let wrap = x + m * y;
                    let taken = is_wall[wrap];
                    if taken {
                        answer.push([x + 1, y + 1]);
                    }
                    let r1 = reachable[2 * wrap];
                    let r2 = reachable[2 * wrap + 1];
                    let d = taken as u8 * 4 + r1 as u8 * 2 + r2 as u8;
                    #[cfg(test)]
                    if is_mountain[wrap] {
                        print!(".");
                    } else {
                        print!("{d}");
                    }
                }
                #[cfg(test)]
                println!();
            }
            // dbg!(&graph, flow, &reachable, wrap_s, wrap_t, &answer);
            assert_eq!(flow as usize, answer.len());
            Some(answer)
        }
        Infinite => None,
    }
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
        assert_eq!(None, actual);
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
        assert_eq!(Some(vec![]), actual);
    }

    #[test]
    fn test_my1() {
        // a.b
        let actual = solve([3, 1], [1, 1], [3, 1], &[], &[[2, 1]]);
        assert_eq!(Some(vec![[2, 1]]), actual);
    }

    #[test]
    fn test_my2() {
        // a.
        // .b
        let actual = solve([2, 2], [1, 1], [2, 2], &[], &[[1, 2], [2, 1]]);
        assert_eq!(Some(vec![[2, 1], [1, 2]]), actual);
    }

    #[test]
    fn test_my3() {
        // (45, 2, [], [(1, 1), (2, 2)], (2, 1), (1, 2))
        solve([2, 2], [2, 1], [1, 2], &[], &[[1, 1], [2, 2]]);
    }

    #[test]
    fn test_my4() {
        // (5, 7, [], [(1, 2), (1, 4), (1, 5), (1, 6), (1, 7), (2, 1), (2, 2)], (1, 3), (1, 1))
        solve(
            [5, 7],
            [1, 3],
            [1, 1],
            &[],
            &[[1, 2], [1, 4], [1, 5], [1, 6], [1, 7], [2, 1], [2, 2]],
        );
    }

    #[test]
    fn test_my5() {
        // (5, 7, [(1, 1), (1, 4), (1, 5)], [(1, 2), (1, 7), (2, 1), (2, 2), (2, 4), (2, 5), (2, 6)], (1, 3), (1, 6))
        solve(
            [5, 7],
            [1, 3],
            [1, 6],
            &[[1, 1], [1, 4], [1, 5]],
            &[[1, 2], [1, 7], [2, 1], [2, 2], [2, 4], [2, 5], [2, 6]],
        );
    }

    #[test]
    fn test_my6() {
        // (5, 7, [], [(1, 3), (1, 2), (1, 5), (1, 6), (1, 7), (2, 2), (2, 5), (2, 3), (2, 4)], (1, 1), (1, 4))
        solve(
            [5, 7],
            [1, 1],
            [1, 4],
            &[],
            &[
                [1, 3],
                [1, 2],
                [1, 5],
                [1, 6],
                [1, 7],
                [2, 2],
                [2, 5],
                [2, 3],
                [2, 4],
            ],
        );
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
                    g.add_edge(wrap1, wrap2, Finite(1));
                }
            }
        }
        for y in 1..n {
            for x in 0..m {
                let wrap1 = x + m * (y - 1);
                let wrap2 = x + m * y;
                if is_passable[wrap1] && is_passable[wrap2] {
                    g.add_edge(wrap1, wrap2, Finite(1));
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
            assert!(actual.is_none());
        } else {
            assert!(actual.is_some());
            let actual = actual.unwrap();
            assert!(actual.len() <= expected as usize);
        }
    }

    proptest! {
        #[test]
        fn test_props(input in gen_input([10, 10])) {
            compare_with_baseline(input);
        }
    }
}
