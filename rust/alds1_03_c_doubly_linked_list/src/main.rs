// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_3_C

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
    let n = scan!(usize);
    let mut v: VecDeque<usize> = VecDeque::new();
    let input = scan!(String;; n);

    for i in input {
        match i[0].as_str() {
            "insert" => v.push_front(i[1].parse().expect("Expected an integer")),
            "delete" => {
                let x = i[1].parse::<usize>().expect("Expected an integer");
                let index = v.iter().position(|&val| x == val).unwrap_or(v.len());
                let mut tail = v.split_off(index);
                tail.pop_front();
                v.append(&mut tail);
            }
            "deleteFirst" => {
                v.pop_front();
            }
            "deleteLast" => {
                v.pop_back();
            }
            _ => panic!("Invalid input"),
        }
    }

    println!(
        "{}",
        v.iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
