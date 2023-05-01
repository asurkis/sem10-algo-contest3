use util::debug;

fn main() {
    let mut lines = std::io::stdin().lines().map(|s| {
        s.unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>()
    });
    let nm = lines.next().unwrap();
    let m = nm[0];
    let cubes = lines.next().unwrap();
    let answer = solve(m, &cubes);
    for x in answer {
        print!("{x} ");
    }
    println!();
}

fn solve(_m: usize, cubes: &[usize]) -> Vec<usize> {
    let n = cubes.len();
    let mut max_paly = vec![0; n];
    let mut c = 0;
    let mut r = 0;
    for i in 0..n {
        let mut k = 0;
        if i < r {
            let j = c + c - i;
            k = max_paly[j].min(r - i);
        }
        for j in k..i {
            if i + j >= n {
                break;
            } else if cubes[i + j] == cubes[i - j - 1] {
                k += 1;
            } else {
                break;
            }
        }
        if i + k > r {
            c = i;
            r = i + k;
        }
        max_paly[i] = k;
    }

    debug!(&max_paly);

    let mut answer = vec![];
    for i in 0..n {
        if max_paly[i] == i {
            answer.push(n - i);
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    fn _baseline(_m: usize, cubes: &[usize]) -> Vec<usize> {
        let n = cubes.len();
        let mut max_paly = vec![0; n];
        for i in 0..n {
            for j in 0..i {
                if i + j >= n {
                    break;
                }
                if cubes[i + j] == cubes[i - j - 1] {
                    max_paly[i] += 1;
                } else {
                    break;
                }
            }
        }

        let mut answer = vec![];
        for i in 0..n {
            if max_paly[i] == i {
                answer.push(n - i);
            }
        }
        answer
    }

    #[test]
    fn test1() {
        // 6 2
        // 1 1 2 2 1 1
        // ========
        // 6 5 3
        let cubes = vec![1, 1, 2, 2, 1, 1];
        let actual = solve(2, &cubes);
        let expected = vec![6, 5, 3];
        assert_eq!(expected, actual);
    }
}
