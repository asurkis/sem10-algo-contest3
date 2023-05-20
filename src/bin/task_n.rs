use util::*;

fn main() {
    let mut lines = std::io::stdin().lines();
    let line1 = lines.next().unwrap().unwrap();
    let line2 = lines.next().unwrap().unwrap();
    let answer = solve(line1.trim(), line2.trim());
    println!("{answer}");
}

fn solve(a_str: &str, b_str: &str) -> String {
    let n = a_str.len();
    let m = b_str.len();
    if n < m {
        return solve(b_str, a_str);
    }
    let sa1: Vec<char> = a_str.chars().collect();
    let sb1: Vec<char> = b_str.chars().collect();
    let sa2: Vec<char> = a_str.chars().rev().collect();
    let sb2: Vec<char> = b_str.chars().rev().collect();
    let mut pat1 = vec![' '; n + m];
    let mut pat2 = vec![' '; n + m];
    pat1[..m].copy_from_slice(&sb1);
    pat2[..m].copy_from_slice(&sb2);
    pat1[m..].copy_from_slice(&sa1);
    pat2[m..].copy_from_slice(&sa2);

    let zfun1 = calc_zfun(&pat1);
    let zfun2 = calc_zfun(&pat2);

    let pat1s: String = pat1.iter().collect();
    let pat2s: String = pat2.iter().collect();
    debug!(a_str; b_str; pat1s; pat2s; &zfun1; &zfun2);

    // debug!(&pat1; &zfun1);
    for i in 0..n {
        // debug!(i, m + i, zfun1[m + i], m);
        if zfun1[m + i] >= m {
            return find_period(&sa1);
        }
    }

    let cand1 = try_find(&sa1, &sb1, &zfun1);
    let cand2 = try_find(&sa2, &sb2, &zfun2);
    let cand3 = find_period(&pat1);
    let cand4 = find_period(&pat2);
    // debug!(&cand1; &cand2; &cand3; &cand4);

    let minlen = cand1
        .len()
        .min(cand2.len())
        .min(cand3.len())
        .min(cand4.len());

    if minlen == cand1.len() {
        cand1
    } else if minlen == cand3.len() {
        cand3
    } else {
        if minlen == cand2.len() { cand2 } else { cand4 }
            .chars()
            .rev()
            .collect()
    }
}

fn try_find(sa: &[char], sb: &[char], zfun: &[usize]) -> String {
    let n = sa.len();
    let m = sb.len();
    assert_eq!(n + m, zfun.len());
    let mut cand = Vec::new();
    let c1: String = sa.iter().collect();
    let c2: String = sb.iter().collect();
    debug!(c1, c2; zfun);
    for i in 0..n {
        let len = zfun[m + i].min(m);
        if i + len == n {
            debug!(i, len, m, n);
            cand = vec![' '; n + m - len];
            cand[0..n].copy_from_slice(&sa);
            cand[n..n + m - len].copy_from_slice(&sb[len..m]);
            break;
        }
    }
    if cand.len() == 0 {
        cand = vec![' '; n + m];
        cand[0..n].copy_from_slice(&sa);
        cand[n..n + m].copy_from_slice(&sb);
    }
    let c3: String = cand.iter().collect();
    debug!(c3);
    find_period(&cand)
}

fn find_period(s: &[char]) -> String {
    let n = s.len();
    let zfun = calc_zfun(s);
    // debug!(s; &zfun);
    for i in 1..n {
        if i + zfun[i] == n {
            // debug!(i, &s[0..i]);
            return s[0..i].iter().collect();
        }
    }
    s.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // ababa
        // bab
        // ========
        // ab
        let answer = solve("ababa", "bab");
        debug!(answer);
        assert!(answer == "ab" || answer == "ba");
    }

    #[test]
    fn test2() {
        // a
        // b
        // ========
        // ba
        let answer = solve("a", "b");
        debug!(answer);
        assert!(answer == "ab" || answer == "ba");
    }

    #[test]
    fn test_my1() {
        let answer = solve("abcd", "bcde");
        debug!(answer);
        assert!(answer == "abcde");
    }

    #[test]
    fn test_my2() {
        let answer = solve("bcde", "abcd");
        debug!(answer);
        assert!(answer == "bcdea");
    }
}
