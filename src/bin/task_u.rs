use util::calc_zfun;

fn main() {
    let line = std::io::stdin().lines().next().unwrap().unwrap();
    let answer = solve(line.trim());
    println!("{answer}");
}

fn solve(input: &str) -> usize {
    let s: Vec<char> = input.chars().collect();
    let n = s.len();
    let zfun = calc_zfun(&s);
    for i in 1..n {
        if i + zfun[i] == n {
            return i;
        }
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let actual = solve("zzz");
        assert_eq!(1, actual);
    }

    #[test]
    fn test2() {
        let actual = solve("bcabcab");
        assert_eq!(3, actual);
    }
}
