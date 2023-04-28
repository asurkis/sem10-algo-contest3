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

    let mut sorted = vec![0; n];
    let mut ord_cls = vec![0; n];
    sort_init_char(input, &mut sorted, &mut ord_cls);

    let mut suffix_pos = vec![0; n];
    let mut ord_cls_out = vec![0; n];
    let mut sorted_out = vec![0; n];

    let mut count = vec![0; n];
    let mut end = vec![0; n];
    let mut ord_val = vec![0; n];

    let mut k = 0;
    while 1 << k < n {
        for i in 0..n {
            suffix_pos[sorted[i]] = i;
        }
        debug!(&sorted);
        debug!(&suffix_pos);
        debug!(&ord_cls);
        let mut ord_cls_next = 0;
        let mut eq_begin = 0;
        for i in 1..n {
            if ord_cls[i] == ord_cls[eq_begin] {
                continue;
            }
            sort_step(
                1 << k,
                &suffix_pos,
                &ord_cls,
                &sorted[eq_begin..i],
                &mut count,
                &mut end,
                &mut ord_val,
                &mut sorted_out[eq_begin..i],
                &mut ord_cls_out[eq_begin..i],
                &mut ord_cls_next,
            );
            eq_begin = i;
        }
        sort_step(
            1 << k,
            &suffix_pos,
            &ord_cls,
            &sorted[eq_begin..n],
            &mut count,
            &mut end,
            &mut ord_val,
            &mut sorted_out[eq_begin..n],
            &mut ord_cls_out[eq_begin..n],
            &mut ord_cls_next,
        );
        swap(&mut sorted, &mut sorted_out);
        swap(&mut ord_cls, &mut ord_cls_out);
        k += 1;
    }
    debug!(&sorted; &ord_cls);

    for i in 0..n {
        if sorted[i] == 0 {
            return i + 1;
        }
    }
    panic!();
}

fn sort_step(
    offset: usize,
    suffix_pos: &[usize],
    ord_cls: &[usize],
    sorted: &[usize],
    count: &mut [usize],
    end: &mut [usize],
    ord_val: &mut [usize],
    sorted_out: &mut [usize],
    ord_cls_out: &mut [usize],
    ord_cls_next: &mut usize,
) {
    let n = suffix_pos.len();
    let m = sorted.len();
    debug_assert_eq!(n, ord_cls.len());
    debug_assert_eq!(n, count.len());
    debug_assert_eq!(n, end.len());
    debug_assert_eq!(n, ord_val.len());
    debug_assert_eq!(m, sorted.len());
    debug_assert_eq!(m, sorted_out.len());
    debug_assert_eq!(m, ord_cls_out.len());
    if m == 0 {
        return;
    }
    if m == 1 {
        sorted_out[0] = sorted[0];
        ord_cls_out[0] = *ord_cls_next;
        *ord_cls_next += 1;
        return;
    }

    for i in 0..n {
        count[i] = 0;
    }
    for i in 0..m {
        let j = suffix_pos[(sorted[i] + offset) % n];
        count[ord_cls[j]] += 1;
    }

    end[0] = 0;
    ord_val[0] = *ord_cls_next;
    for i in 0..n - 1 {
        end[i + 1] = end[i] + count[i];
        ord_val[i + 1] = ord_val[i] + if count[i] != 0 { 1 } else { 0 };
    }
    debug_assert_eq!(m, end[n - 1] + count[n - 1]);
    *ord_cls_next = ord_val[n - 1] + if count[n - 1] != 0 { 1 } else { 0 };

    for i in 0..m {
        let j = suffix_pos[(sorted[i] + offset) % n];
        let ord = ord_cls[j];
        debug!(i, j, ord);
        sorted_out[end[ord]] = sorted[i];
        ord_cls_out[end[ord]] = ord_val[ord];
        end[ord] += 1;
    }
}

fn sort_init_char(s: &str, sorted: &mut [usize], ord_cls: &mut [usize]) {
    let n = s.len();
    // stable count sort by first character
    let mut char_count = vec![0; 256];
    for c in s.chars() {
        char_count[c as usize] += 1;
    }
    let mut char_begin = vec![0; 256];
    let mut next_ord_cls = 0;
    for i in 0..255 {
        char_begin[i + 1] = char_begin[i] + char_count[i];
        if char_count[i] != 0 {
            for j in char_begin[i]..char_begin[i + 1] {
                ord_cls[j] = next_ord_cls;
            }
            next_ord_cls += 1;
        }
    }
    for j in char_begin[255]..n {
        ord_cls[j] = next_ord_cls;
    }
    let mut char_end = char_begin.clone();
    for (i, c) in s.chars().enumerate() {
        let ce = &mut char_end[c as usize];
        sorted[*ce] = i;
        *ce += 1;
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
