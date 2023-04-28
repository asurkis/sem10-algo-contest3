use math::*;
use std::mem::swap;

#[cfg(test)]
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
macro_rules! debug {
    ($($($val:expr),+);*) => {};
}

fn main() {
    let line = std::io::stdin().lines().next().unwrap().unwrap();
    let answer = solve(line.trim());
    println!("{answer}");
}

fn solve(input: &str) -> usize {
    let n = input.len();
    let s: Vec<char> = input.chars().collect();
    let mut sorted = vec![0; n];
    let mut sorted_out = vec![0; n];
    let mut ord = vec![0; n];
    let mut ord_out = vec![0; n];
    sort_init_char(&s, &mut sorted, &mut ord);

    debug!(&sorted; &ord);

    let mut k = 0;
    while 1 << k < n {
        sort_step(1 << k, &sorted, &ord, &mut sorted_out, &mut ord_out);
        swap(&mut sorted, &mut sorted_out);
        swap(&mut ord, &mut ord_out);
        debug!(&sorted; &ord);
        let mut ord_sorted = ord.clone();
        ord_sorted.sort();
        debug!(ord_sorted);
        k += 1;
    }

    for i in 0..n {
        if sorted[i] == 0 {
            return i + 1;
        }
    }
    panic!();
}

fn calc_zfun(s: &[char], z: &mut [usize]) {
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

fn sort_step(
    offset: usize,
    sorted: &[usize],
    ord: &[usize],
    sorted_out: &mut [usize],
    ord_out: &mut [usize],
) {
    let n = sorted.len();
    debug_assert_eq!(n, sorted_out.len());
    debug_assert_eq!(n, ord.len());
    debug_assert_eq!(n, ord_out.len());

    sorted_out.copy_from_slice(sorted);
    let tmp: Vec<_> = sorted_out
        .iter()
        .map(|&i| (i, (i + offset) % n, ord[i], ord[(i + offset) % n]))
        .collect();
    debug!(&tmp);
    sorted_out.sort_by_key(|&i| (ord[i], ord[(i + offset) % n], i));
    let tmp: Vec<_> = sorted_out
        .iter()
        .map(|&i| (i, (i + offset) % n, ord[i], ord[(i + offset) % n]))
        .collect();
    debug!(&tmp);
    ord_out[sorted_out[0]] = 0;
    for i in 1..n {
        let j = sorted_out[i - 1];
        let k = sorted_out[i];
        let oj = (ord[j], ord[(j + offset) % n]);
        let ok = (ord[k], ord[(k + offset) % n]);
        ord_out[k] = ord_out[j] + if ok != oj { 1 } else { 0 };
        debug!(j, k, oj, ok, ord_out[k]);
    }
    /*
    let mut count = vec![0; n];
    for i in 0..n {
        count[ord[i]] += 1;
    }

    let mut end = Vec::with_capacity(n);
    end.push(0);
    for i in 0..n - 1 {
        end.push(end[i] + count[i]);
    }

    let mut sort1 = vec![0; n];
    for i in 0..n {
        let j = (sorted[i] + offset) % n;
        let o = ord[j];
        sort1[end[o]] = sorted[i];
    }

    for i in 0..n {
        let j = sort1[i];
        sorted_out[end[j]];
    }
    */
}

fn sort_init_char(s: &[char], sorted_out: &mut [usize], ord_out: &mut [usize]) {
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
        sorted_out[char_end[cu]] = i;
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

    use proptest::prelude::*;
    proptest! {
        #[test]
        fn test_props(input in "[a-z]{10,}") {
            compare_with_baseline(&input);
        }
    }
}

#[allow(unused)]
mod math {
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
}
