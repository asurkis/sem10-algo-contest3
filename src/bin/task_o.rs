fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let (cost, answer) = solve(line.trim());
    println!("{cost} {}", answer.len());
    for cmd in answer {
        match cmd {
            Char(c) => println!("{c}"),
            Repeat(j, p) => println!("{j} {p}"),
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SeqElem {
    Char(char),
    Repeat(usize, usize),
}
use SeqElem::*;

impl SeqElem {
    fn cost(self) -> u64 {
        match self {
            Char(_) => 1,
            Repeat(_, _) => 5,
        }
    }
}

use util::{calc_pfun_inplace, debug};

fn solve(string: &str) -> (u64, Vec<SeqElem>) {
    let n = string.len();
    let chars: Vec<char> = string.chars().collect();
    let mut pfuns = Vec::with_capacity(n);
    for i in 0..n {
        let mut pfun1 = vec![0; n];
        calc_pfun_inplace(&chars[i..n], &mut pfun1[i..n]);
        pfuns.push(pfun1);
    }
    let pfuns = pfuns;

    let mut best = Vec::with_capacity(n);
    best.push((1, Char(chars[0])));
    for i in 1..n {
        let mut best_cmd = Char(chars[i]);
        let mut best_cost = best[i - 1].0 + best_cmd.cost();
        for j in 0..i {
            let p = pfuns[j][i];
            if p == 0 {
                continue;
            }
            let cmd = Repeat(j + 1, p);
            let cost = best[i - p].0 + cmd.cost();
            if best_cost > cost {
                best_cost = cost;
                best_cmd = cmd;
            }
        }
        best.push((best_cost, best_cmd));
    }

    // debug!(&best);

    let mut answer = Vec::new();
    let mut i = n;
    while i != 0 {
        // let i_old = i;
        let cmd = best[i - 1].1;
        answer.push(cmd);
        match cmd {
            Char(_) => i -= 1,
            Repeat(_, j) => i -= j,
        };
        // debug!(i_old, cmd, i);
    }

    answer.reverse();
    (best[n - 1].0, answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // abcdqwertyqwertyu
        // ========
        // 16 12
        // a
        // b
        // c
        // d
        // q
        // w
        // e
        // r
        // t
        // y
        // 5 6
        // u
        let answer = solve("abcdqwertyqwertyu");
        let expected = vec![
            Char('a'),
            Char('b'),
            Char('c'),
            Char('d'),
            Char('q'),
            Char('w'),
            Char('e'),
            Char('r'),
            Char('t'),
            Char('y'),
            Repeat(5, 6),
            Char('u'),
        ];
        assert_eq!((16, expected), answer);
    }

    #[test]
    fn test2() {
        // aaaaaaa
        // ========
        // 6 2
        // a
        // 1 6
        let answer = solve("aaaaaaa");
        let expected = vec![Char('a'), Repeat(1, 6)];
        assert_eq!((6, expected), answer);
    }
}
