use std::mem::swap;

fn main() {
    let line = std::io::stdin().lines().next().unwrap().unwrap();
    let answer = solve(line.trim());
    println!("{answer}");
}

fn solve(input: &str) -> usize {
    let n = input.len();
    let s: Vec<char> = input.chars().collect();
    let mut sorted = vec![0; n];
    let mut ord = vec![0; n];
    let mut ord_out = vec![0; n];
    sort_init_char(&s, &mut sorted, &mut ord);

    let mut buf1 = vec![0; n];
    let mut buf2 = vec![0; n];
    let mut buf3 = vec![0; n];

    let mut sorted_prev = sorted.clone();
    let mut k = 0;
    while 1 << k < n {
        sort_step(
            1 << k,
            &ord,
            &mut sorted,
            &mut ord_out,
            &mut buf1,
            &mut buf2,
            &mut buf3,
        );
        swap(&mut ord, &mut ord_out);
        if sorted == sorted_prev {
            break;
        }
        sorted_prev.copy_from_slice(&sorted);
        k += 1;
    }

    for i in 0..n {
        if sorted[i] == 0 {
            return i + 1;
        }
    }
    panic!();
}

fn sort_step(
    offset: usize,
    ord: &[u32],
    sorted: &mut [u32],
    ord_out: &mut [u32],
    sorted_buf: &mut [u32],
    count: &mut [u32],
    end: &mut [u32],
) {
    let n = sorted.len();
    debug_assert_eq!(n, sorted_buf.len());
    debug_assert_eq!(n, ord.len());
    debug_assert_eq!(n, ord_out.len());

    let ord_max = ord[sorted[n - 1] as usize] as usize;
    count[..=ord_max].fill(0);
    for i in 0..n {
        count[ord[i] as usize] += 1;
    }

    end[0] = 0;
    for i in 1..=ord_max {
        end[i] = end[i - 1] + count[i - 1];
    }

    for i in 0..n {
        let o = ord[(i + offset) % n] as usize;
        sorted_buf[end[o] as usize] = i as u32;
        end[o] += 1;
    }

    end[0] = 0;
    for i in 1..=ord_max {
        end[i] = end[i - 1] + count[i - 1];
    }

    for i in 0..n {
        let o = ord[sorted_buf[i] as usize] as usize;
        sorted[end[o] as usize] = sorted_buf[i];
        end[o] += 1;
    }

    // sorted_out.copy_from_slice(sorted);
    // sorted_out.sort_by_key(|&i| (ord[i], ord[(i + offset) % n], i));
    ord_out[sorted[0] as usize] = 0;
    for i in 1..n {
        let j = sorted[i - 1] as usize;
        let k = sorted[i] as usize;
        let oj = (ord[j], ord[(j + offset) % n]);
        let ok = (ord[k], ord[(k + offset) % n]);
        ord_out[k] = ord_out[j] + if ok != oj { 1 } else { 0 };
    }
}

fn sort_init_char(s: &[char], sorted_out: &mut [u32], ord_out: &mut [u32]) {
    let n = s.len();
    // stable count sort by first character
    let mut char_count = vec![0; 256];
    for &c in s {
        char_count[c as usize] += 1;
    }
    let mut char_end = vec![0; 256];
    let mut ord_val = vec![0; 256];
    for i in 0..255 {
        char_end[i + 1] = char_end[i] + char_count[i];
        ord_val[i + 1] = ord_val[i] + if char_count[i] != 0 { 1 } else { 0 };
    }
    for i in 0..n {
        let cu = s[i] as usize;
        sorted_out[char_end[cu]] = i as u32;
        ord_out[i] = ord_val[cu];
        char_end[cu] += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn baseline(input: &str) -> usize {
        let n = input.len();
        let chars: Vec<char> = input.chars().collect();
        let mut arr = Vec::with_capacity(n);
        for i in 0..n {
            let mut v = Vec::with_capacity(n);
            for j in 0..n {
                v.push(chars[(i + j) % n]);
            }
            arr.push((v, i));
        }
        arr.sort();
        for i in 0..n {
            if arr[i].1 == 0 {
                return i + 1;
            }
        }
        panic!();
    }

    fn compare_with_baseline(input: &str) {
        let expected = baseline(input);
        let actual = solve(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_abracadabra() {
        // | 10 -> 0 |  0 ->  | aabracadabr
        // |  0 -> 1 |  1 ->  | abracadabra
        // |  7 -> 1 |  8 ->  | abraabracad
        // |  3 -> 2 |  4 ->  | acadabraabr
        // |  5 -> 3 |  6 ->  | adabraabrac
        // |  1 -> 4 |  2 ->  | bracadabraa
        // |  8 -> 4 |  9 ->  | braabracada
        // |  4 -> 5 |  5 ->  | cadabraabra
        // |  6 -> 6 |  7 ->  | dabraabraca
        // |  2 -> 7 |  3 ->  | racadabraab
        // |  9 -> 7 | 10 ->  | raabracadab
        let input = "abracadabra";
        let expected = 3;
        let actual = solve(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_cabcab() {
        let input = "cabcab";
        let expected = 5;
        let actual = solve(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test1() {
        // bbbc
        // bbcb
        // bcbb
        // cbbb
        compare_with_baseline("bbcb");
    }

    #[test]
    fn test2() {
        // bbcd
        // bcdb
        // cdbb
        // dbbc
        compare_with_baseline("bcdb");
    }

    #[test]
    fn test_million() {
        let input: String = (0..1_000_000).map(|_| 'x').collect();
        let actual = solve(&input);
        assert_eq!(1, actual);
    }

    use proptest::prelude::*;
    proptest! {
        #[test]
        fn test_props(input in "[a-z]{10,}") {
            compare_with_baseline(&input);
        }
    }
}
