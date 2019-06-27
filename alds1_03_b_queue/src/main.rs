// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_3_B

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
    let (n, q) = scan!(usize, isize);
    let process = scan!(String, isize; n);
    let mut timestamp = 0;
    let mut queue = VecDeque::from(process);

    while let Some((p, t)) = queue.pop_front() {
        if t > q {
            timestamp += q;
            queue.push_back((p, t - q));
        } else {
            timestamp += t;
            println!("{} {}", p, timestamp);
        }
    }
}
