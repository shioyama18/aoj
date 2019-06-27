// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_4_B

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

use std::cmp::Ordering;

fn binary_search(s: &[usize], i: usize) -> usize {
    if s.is_empty() {
        return 0;
    }

    let n = s.len();
    let mid_index = n / 2;
    let mid = s[mid_index];

    match mid.cmp(&i) {
        Ordering::Less => binary_search(&s[(mid_index + 1)..n], i),
        Ordering::Equal => 1,
        Ordering::Greater => binary_search(&s[0..mid_index], i),
    }
}

fn main() {
    let _n = scan!(usize);
    let s = scan!(usize ;;);
    let _q = scan!(usize);
    let t = scan!(usize ;;);

    let mut c: usize = 0;
    for i in t {
        c += binary_search(&s, i);
    }

    println!("{}", c);
}
