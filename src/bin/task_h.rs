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

    let max_wins = current_score[0] + coming_games[0];
    let mut graph = Graph::new(2 * n);
    let s = 0;
    let t = 1;
    let mut total = 0;

    for i in 1..n {
        if current_score[i] > max_wins {
            return false;
        }
        let allowed_wins = max_wins - current_score[i];
        let mut free_matches = 0;

        for j in 1..n {
            let games_ij = games_mat[i][j];
            graph.add_edge(2 * i, 2 * j + 1, Finite(games_ij));
            free_matches += games_ij;
        }

        graph.add_edge(s, 2 * i, Finite(allowed_wins));
        graph.add_edge(2 * i + 1, t, Finite(free_matches));
        total += free_matches;
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
