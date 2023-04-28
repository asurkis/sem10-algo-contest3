use std::mem::swap;

#[cfg(test)]
macro_rules! debug {
    ($($val:expr);*) => {
        $(
            eprintln!(
                "[{}:{}] {} = {:?}",
                file!(),
                line!(),
                stringify!($val),
                $val
            );
        )*
    };
}

#[cfg(not(test))]
macro_rules! debug {
    ($($val:expr);*) => {};
}

fn main() {
    let line = std::io::stdin().lines().next().unwrap().unwrap();
    let answer = baseline(line.trim());
    println!("{answer}");
}

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

fn solve(input: &str) -> usize {
    let n = input.len();

    let mut sorted = vec![0; n];
    let mut ord_cls = vec![0; n];
    sort_init_char(input, &mut sorted, &mut ord_cls);

    let mut suffix_pos = vec![0; n];
    let mut ord_cls_out = vec![0; n];
    let mut sorted_out = vec![0; n];

    let mut k = 0;
    while 1 << k < n {
        for i in 0..n {
            suffix_pos[sorted[i]] = i;
        }
        debug!(&sorted; &suffix_pos; &ord_cls);
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
            &mut sorted_out[eq_begin..n],
            &mut ord_cls_out[eq_begin..n],
            &mut ord_cls_next,
        );
        swap(&mut sorted, &mut sorted_out);
        swap(&mut ord_cls, &mut ord_cls_out);
        k += 1;
    }
    debug!(&sorted; &ord_cls);

    suffix_pos[0] + 1
}

fn sort_step(
    offset: usize,
    suffix_pos: &[usize],
    ord_cls: &[usize],
    sorted: &[usize],
    sorted_out: &mut [usize],
    ord_cls_out: &mut [usize],
    ord_cls_next: &mut usize,
) {
    let n = suffix_pos.len();
    let m = sorted.len();
    debug_assert_eq!(n, ord_cls.len());
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

    let nn = 256.max(n);
    let mut count = vec![0; nn];
    for i in 0..m {
        let j = suffix_pos[(i + offset) % n];
        count[ord_cls[j]] += 1;
    }

    let mut end = vec![0; nn];
    let mut ord_n = vec![*ord_cls_next; nn];
    for i in 0..nn - 1 {
        end[i + 1] = end[i] + count[i];
        ord_n[i + 1] = ord_n[i] + if count[i] != 0 { 1 } else { 0 };
    }
    debug_assert_eq!(m, end[nn - 1] + count[nn - 1]);
    *ord_cls_next = ord_n[nn - 1] + if count[nn - 1] != 0 { 1 } else { 0 };

    for i in 0..m {
        let j = suffix_pos[(i + offset) % n];
        let ord = ord_cls[j];
        sorted_out[end[ord]] = sorted[i];
        ord_cls_out[end[ord]] = ord_n[ord];
        end[ord] += 1;
    }
}

fn sort_init_char(s: &str, sorted: &mut [usize], ord_cls: &mut [usize]) {
    // stable count sort by first character
    let mut char_count = vec![0; 256];
    for c in s.chars() {
        char_count[c as usize] += 1;
    }
    let mut char_begin = vec![0; 256];
    for i in 0..255 {
        char_begin[i + 1] = char_begin[i] + char_count[i];
        for j in char_begin[i]..char_begin[i + 1] {
            ord_cls[j] = i;
        }
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
}
