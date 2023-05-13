use util::{Capacity::*, Graph};

type Int = u64;

fn main() {
    let lines: Vec<Vec<Int>> = std::io::stdin()
        .lines()
        .map(|s| {
            s.unwrap()
                .trim()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<Int>>()
        })
        .collect();
    let n = lines[0][0] as usize;
    let current_score = &lines[1];
    let coming_games = &lines[2];
    let games_mat = &lines[3..3 + n];
    let answer = solve(n, current_score, coming_games, games_mat);
    if answer {
        println!("YES");
    } else {
        println!("NO");
    }
}

fn solve(n: usize, current_score: &[Int], coming_games: &[Int], games_mat: &[Vec<Int>]) -> bool {
    debug_assert_eq!(n, current_score.len());
    debug_assert_eq!(n, coming_games.len());
    debug_assert_eq!(n, games_mat.len());
    for i in 0..n {
        debug_assert_eq!(n, games_mat[i].len());
    }

    // (n - 1) * (n - 2) / 2 + 2 * (n - 1) = (n - 1) * (n + 2) / 2
    // 1 0 => 0
    // 2 0 => 1
    // 2 1 => 2
    // 3 0 => 3
    // 3 1 => 4
    // ...

    let s = (n - 1) * (n + 2) / 2;
    let t = s + 1;
    let mut graph = Graph::new(t + 1);

    let max_wins = current_score[0] + coming_games[0];
    let mut total = 0;

    for i in 1..n {
        if current_score[i] > max_wins {
            return false;
        }

        let allowed_wins = max_wins - current_score[i];
        graph.add_edge(i - 1, t, Finite(allowed_wins));

        for j in 1..i {
            let wrap_edge = (i - 1) * (i - 2) / 2 + j - 1;
            let ev = wrap_edge + n - 1;
            let games_ij = games_mat[i][j];
            graph.add_edge(s, ev, Finite(games_ij));
            graph.add_edge(ev, i - 1, Infinite);
            graph.add_edge(ev, j - 1, Infinite);
            total += games_ij;
        }
    }

    let max_flow = graph.max_flow(s, t);
    max_flow == Finite(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // 3
        // 1 2 2
        // 1 1 1
        // 0 0 0
        // 0 0 0
        // 0 0 0
        // ========
        // YES
        let answer = solve(
            3,
            &[1, 2, 2],
            &[1, 1, 1],
            &[vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]],
        );
        assert!(answer);
    }

    #[test]
    fn test2() {
        // 3
        // 1 2 2
        // 1 1 1
        // 0 0 0
        // 0 0 1
        // 0 1 0
        // ========
        // NO
        let answer = solve(
            3,
            &[1, 2, 2],
            &[1, 1, 1],
            &[vec![0, 0, 0], vec![0, 0, 1], vec![0, 1, 0]],
        );
        assert!(!answer);
    }
}
