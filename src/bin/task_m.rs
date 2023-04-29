fn main() {
    let mut lines = std::io::stdin().lines().map(|s| {
        s.unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>()
    });
    let nm = lines.next().unwrap();
    let m = nm[0];
    let cubes = lines.next().unwrap();
    let answer = solve(m, &cubes);
    for x in answer {
        print!("{x} ");
    }
    println!();
}

fn solve(_m: usize, cubes: &[usize]) -> Vec<usize> {
    let n = cubes.len();
    let mut max_paly = vec![0; n];
    let mut c = 0;
    let mut r = 0;
    for i in 0..n {
        let mut k = 0;
        if i < r {
            let j = c + c - i;
            k = max_paly[j].min(r - i);
        }
        for j in k..i {
            if i + j >= n {
                break;
            } else if cubes[i + j] == cubes[i - j - 1] {
                k += 1;
            } else {
                break;
            }
        }
        if i + k > r {
            c = i;
            r = i + k;
        }
        max_paly[i] = k;
    }

    debug!(&max_paly);

    let mut answer = vec![];
    for i in 0..n {
        if max_paly[i] == i {
            answer.push(n - i);
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    fn _baseline(_m: usize, cubes: &[usize]) -> Vec<usize> {
        let n = cubes.len();
        let mut max_paly = vec![0; n];
        for i in 0..n {
            for j in 0..i {
                if i + j >= n {
                    break;
                }
                if cubes[i + j] == cubes[i - j - 1] {
                    max_paly[i] += 1;
                } else {
                    break;
                }
            }
        }

        let mut answer = vec![];
        for i in 0..n {
            if max_paly[i] == i {
                answer.push(n - i);
            }
        }
        answer
    }

    #[test]
    fn test1() {
        // 6 2
        // 1 1 2 2 1 1
        // ========
        // 6 5 3
        let cubes = vec![1, 1, 2, 2, 1, 1];
        let actual = solve(2, &cubes);
        let expected = vec![6, 5, 3];
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
