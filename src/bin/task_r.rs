use util::calc_zfun;

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
    // a b a c a b a a
    // 0 0 1 0 1 2 3 1
    // 8 0 1 0 3 0 1 1
    let mut new_char = 1;
    let mut string = vec![0; n];
    for i in 1..n {
        if pfun[i] == 0 {
            string[i] = new_char;
            new_char += 1;
        } else {
            string[i] = string[pfun[i] - 1];
        }
    }
    let zfun = calc_zfun(&string);
    zfun
}

#[cfg(test)]
mod tests {
    use super::*;

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
