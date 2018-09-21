macro_rules! scan {
    ($t:ty) => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<$t>().unwrap()
        }
    };
    ($($t:ty),*) => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                $(iter.next().unwrap().parse::<$t>().unwrap(),)*
            )
        }
    };
    ($t:ty; $n:expr) => {
        (0..$n).map(|_|
                    scan!($t)
        ).collect::<Vec<_>>()
    };
    ($($t:ty),*; $n:expr) => {
        (0..$n).map(|_|
                    scan!($($t),*)
        ).collect::<Vec<_>>()
    };
    ($t:ty ;;) => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.split_whitespace()
                .map(|t| t.parse::<$t>().unwrap())
                .collect::<Vec<_>>()
        }
    };
    ($t:ty ;; $n:expr) => {
        (0..$n).map(|_| scan!($t ;;)).collect::<Vec<_>>()
    };
}

use std::collections::HashSet;

fn main() {
    let (n, e) = scan!(usize, usize);
    let input = scan!(usize, usize; e);

    let mut g = vec![Vec::new(); n];
    let mut g_parent = vec![HashSet::new(); n];

    for (s, t) in input {
        g[s].push(t);
        g_parent[t].insert(s);
    }

    let mut stack: Vec<usize> = (0..n).filter(|&i| g_parent[i].is_empty()).collect();
    let mut order = Vec::new();

    while let Some(i) = stack.pop() {
        order.push(i);

        for &to in &g[i] {
            if g_parent[to].remove(&i) && g_parent[to].is_empty() {
                stack.push(to);
            }
        }
    }

    for i in order {
        println!("{}", i);
    }
}
