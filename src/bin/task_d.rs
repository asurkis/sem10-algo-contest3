use util::Graph;

fn main() {
    let mut lines = std::io::stdin().lines().map(|s| s.unwrap());
    let n: usize = lines.next().unwrap().trim().parse().unwrap();
    let mut orders = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap();
        let split: Vec<&str> = line.trim().split_whitespace().collect();
        let (hour_str, minute_str) = split[0].split_once(':').unwrap();
        let hour = hour_str.parse().unwrap();
        let minute = minute_str.parse().unwrap();
        let sx = split[1].parse().unwrap();
        let sy = split[2].parse().unwrap();
        let tx = split[3].parse().unwrap();
        let ty = split[4].parse().unwrap();
        let order = Order::new(hour, minute, sx, sy, tx, ty);
        orders.push(order);
    }

    let answer = solve(&orders);
    println!("{answer}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Order {
    // u16 достаточно на самом деле
    epoch: u64,
    sx: u64,
    sy: u64,
    tx: u64,
    ty: u64,
}

impl Order {
    fn new(hours: u64, minutes: u64, sx: u64, sy: u64, tx: u64, ty: u64) -> Self {
        Self {
            epoch: 60 * hours + minutes,
            sx,
            sy,
            tx,
            ty,
        }
    }

    fn precedes(self, that: Order) -> bool {
        self.epoch
            + self.tx.abs_diff(self.sx)
            + self.ty.abs_diff(self.sy)
            + self.tx.abs_diff(that.sx)
            + self.ty.abs_diff(that.sy)
            < that.epoch
    }
}

fn solve(orders: &[Order]) -> usize {
    let n = orders.len();
    let mut h = Graph::new(2 * n + 2);
    let s = 2 * n;
    let t = s + 1;
    let mut og_edges = Vec::new();
    for i in 0..n {
        for j in 0..n {
            if orders[i].precedes(orders[j]) {
                let pos = h.add_edge(i, n + j, 1);
                og_edges.push(pos);
            }
        }
    }
    for i in 0..n {
        h.add_edge(s, i, 1);
        // g.add_edge(n + i, i);
        h.add_edge(n + i, t, 1);
    }

    h.max_flow(s, t);

    let mut g = Graph::new(n);
    for ei in og_edges {
        let e = h.edge(ei);
        if e.capacity == 0 {
            g.add_edge(e.node1, e.node2 - n, 1);
        }
    }

    let mut reachable = vec![false; n];
    let mut answer = 0;
    for i in 0..n {
        if reachable[i] {
            continue;
        }
        g.mark_reachable_any(i, &mut reachable);
        answer += 1;
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // 2
        // 08:00 10 11 9 16
        // 08:07 9 16 10 11
        // ========
        // 1
        let actual = solve(&vec![
            Order::new(8, 0, 10, 11, 9, 16),
            Order::new(8, 7, 9, 16, 10, 11),
        ]);
        assert_eq!(1, actual);
    }

    #[test]
    fn test2() {
        // 2
        // 08:00 10 11 9 16
        // 08:06 9 16 10 11
        // ========
        // 2
        let actual = solve(&vec![
            Order::new(8, 0, 10, 11, 9, 16),
            Order::new(8, 6, 9, 16, 10, 11),
        ]);
        assert_eq!(2, actual);
    }
}
