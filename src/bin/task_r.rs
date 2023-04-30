fn main() {
    let mut lines = std::io::stdin().lines().map(|s| {
        s.unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>()
    });
    let n = lines.next().unwrap()[0];
    let pfun = lines.next().unwrap();
    debug_assert_eq!(n, pfun.len());
    let answer = solve(&pfun);
    for &x in &answer {
        print!("{x} ");
    }
    println!();
}

fn solve(pfun: &[usize]) -> Vec<usize> {
    // z[i] = k => p[i + k - 1] >= k
    // p[i] = k => z[i - k + 1] >= k
    let n = pfun.len();
    let mut zfun = vec![0; n];
    // a b a c a b a a
    // 0 0 1 0 1 2 3 1
    // 8 0 1 0 3 0 1 1
    zfun[0] = n;
    for i in 0..n {
        let k = pfun[i];
        let j = i + 1 - k;
        zfun[j] = zfun[j].max(k);
    }
    zfun
}

#[cfg(test)]
mod tests {
    use super::*;

    fn baseline(pfun: &[usize]) -> Vec<usize> {
        todo!()
    }

    fn compare_with_baseline(pfun: &[usize]) {
        let zfun = baseline(pfun);
        let actual = solve(pfun);
        assert_eq!(zfun, actual);
    }

    #[test]
    fn test1() {
        // 8
        // 0 0 1 0 1 2 3 1
        // ========
        // 8 0 1 0 3 0 1 1
        let pfun = vec![0, 0, 1, 0, 1, 2, 3, 1];
        let zfun = vec![8, 0, 1, 0, 3, 0, 1, 1];
        let actual = solve(&pfun);
        assert_eq!(zfun, actual);
    }

    #[test]
    fn test2() {
        // 20
        // 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19
        // ========
        // 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1
        let pfun = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        ];
        let zfun = vec![
            20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1,
        ];
        let actual = solve(&pfun);
        assert_eq!(zfun, actual);
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
