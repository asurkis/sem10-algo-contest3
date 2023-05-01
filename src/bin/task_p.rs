use util::calc_zfun;
use util::debug;

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let answer = solve(line.trim());
    for x in answer {
        print!("{x} ");
    }
    println!();
}

fn solve(input: &str) -> Vec<usize> {
    // forall i : exists j | i - k < j <= i && zfun[j] >= k
    // k -> min
    let s: Vec<char> = input.chars().collect();
    let n = s.len();
    let zfun = calc_zfun(&s);
    debug!(&zfun);
    let mut answer = vec![0; n];
    answer[0] = 1;
    for k in 1..n {
        for cand in 1..=k + 1 {
            if zfun[k + 1 - cand] < cand {
                continue;
            }
            let mut r = 0;
            let mut cand_passes = true;
            for i in 0..=k {
                if zfun[i] >= cand {
                    r = i + cand;
                }
                if i >= r {
                    cand_passes = false;
                    break;
                }
            }
            if cand_passes {
                answer[k] = cand;
                break;
            }
        }
        debug_assert_ne!(0, answer[k]);
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test1() {
        // abaabaababa
        // ========
        // 1 2 3 4 5 3 4 5 3 10 3
        let actual = solve("abaabaababa");
        let expected = vec![1, 2, 3, 4, 5, 3, 4, 5, 3, 10, 3];
        // abaabaababa
        assert_eq!(expected, actual);
    }

    #[test]
    fn test2() {
        // a
        // ========
        // 1
        let actual = solve("a");
        let expected = vec![1];
        assert_eq!(expected, actual);
    }

    fn baseline(input: &str) -> Vec<usize> {
        // forall i : exists j | i - k < j <= i && zfun[j] >= k
        // k -> min
        let s: Vec<char> = input.chars().collect();
        let n = s.len();
        let zfun = calc_zfun(&s);
        debug!(&zfun);
        let mut answer = vec![0; n];
        for k in 0..n {
            for cand in 0..=k {
                if zfun[k - cand] < cand {
                    continue;
                }
                let mut cand_passes = true;
                for i in 0..=k {
                    let j_begin = (cand + 1).max(i + 1) - (cand + 1);
                    let mut i_passes = false;
                    for j in j_begin..=i {
                        if zfun[j] > cand {
                            i_passes = true;
                        }
                    }
                    debug!(cand, j_begin, i, i_passes);
                    if !i_passes {
                        cand_passes = false;
                        break;
                    }
                }
                if cand_passes {
                    answer[k] = cand + 1;
                    break;
                }
            }
            debug_assert_ne!(0, answer[k]);
        }
        answer
    }

    fn compare_with_baseline(input: &str) {
        let expected = baseline(input);
        let answer = solve(input);
        assert_eq!(expected, answer);
    }

    proptest! {
        #[test]
        fn test_props(input in "[a-z]+") {
            compare_with_baseline(&input);
        }
    }
}
