// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_4_C

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
    let n = scan!(usize);
    let input = scan!(String ;; n);

    let mut dict = HashSet::new();

    for i in input {
        match i[0].as_str() {
            "insert" => {
                dict.insert(i[1].clone());
            }
            "find" => {
                if dict.contains(&i[1]) {
                    println!("yes");
                } else {
                    println!("no");
                }
            }
            _ => panic!("Unexpected input"),
        }
    }
}
