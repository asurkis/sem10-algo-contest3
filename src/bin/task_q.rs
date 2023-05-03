fn main() {
    let mut lines = std::io::stdin().lines().map(|s| {
        s.unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>()
    });
    let n = lines.next().unwrap()[0];
    let zfun = lines.next().unwrap();
    debug_assert_eq!(n, zfun.len());
    let answer = solve(&zfun);
    for &x in &answer {
        print!("{x} ");
    }
    println!();
}

fn solve(zfun: &[usize]) -> Vec<usize> {
    let n = zfun.len();
    let mut pfun = vec![0; n];
    let mut j = 0;
    for i in 1..n {
        while j < i && j + zfun[j + 1] < i {
            j += 1;
        }
        pfun[i] = i - j;
    }
    pfun
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused)]
    fn baseline(zfun: &[usize]) -> Vec<usize> {
        let n = zfun.len();
        let mut pfun = vec![0; n];
        for i in 0..n {
            for j in 0..i {
                if j + zfun[j + 1] < i {
                    continue;
                }
                pfun[i] = pfun[i].max(i - j);
            }
        }
        pfun
    }

    #[allow(unused)]
    fn compare_with_baseline(zfun: &[usize]) {
        let pfun = baseline(zfun);
        let actual = solve(zfun);
        assert_eq!(pfun, actual);
    }

    #[test]
    fn test1() {
        // 8
        // 8 0 1 0 3 0 1 1
        // ========
        // 0 0 1 0 1 2 3 1
        let zfun = vec![8, 0, 1, 0, 3, 0, 1, 1];
        let pfun = vec![0, 0, 1, 0, 1, 2, 3, 1];
        let actual = solve(&zfun);
        assert_eq!(pfun, actual);
    }

    #[test]
    fn test2() {
        // 20
        // 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1
        // ========
        // 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19
        let zfun = vec![
            20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1,
        ];
        let pfun = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        ];
        let actual = solve(&zfun);
        assert_eq!(pfun, actual);
    }
}
