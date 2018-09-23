// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_1_A

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

use std::collections::VecDeque;

fn main() {
    let (n, m) = scan!(usize, usize);
    let mut c = String::new();
    std::io::stdin().read_line(&mut c).unwrap();
    let mut c = c.split_whitespace()
        .map(|t| t.parse::<usize>().unwrap())
        .collect::<VecDeque<_>>();
    c.push_front(0);

    let mut t = vec![std::usize::MAX; n + 1];
    t[0] = 0;

    for i in 1..m {
        let mut j = 0;
        while j + c[i] <= n {
            t[j + c[i]] = std::cmp::min(t[j + c[i]], t[j] + 1);
            j += 1;
        }
    }

    println!("{}", t[n]);
}
