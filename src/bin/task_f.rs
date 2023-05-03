use util::{debug, Capacity, Graph};

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
    #[cfg(test)]
    for ei in 0..g.n_edges() {
        debug!(ei, g.edge(ei));
    }

    let mut answer = 0;
    while {
        let subflow = g.mark_subflow(wrap_s, wrap_t);
        if subflow > 2 {
            return -1;
        }
        answer += subflow as i64;
        subflow != 0
    } {}
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

    #[test]
    fn test_my2() {
        // a.
        // .b
        let actual = solve([2, 2], [1, 1], [2, 2], &[], &[[1, 2], [2, 1]]);
        assert_eq!(2, actual);
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
