// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_2_C

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

#[derive(Clone)]
struct Card {
    suit: String,
    value: usize,
}

use std::fmt;

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.suit, self.value)
    }
}

fn print_result(v: &Vec<Card>) {
    println!(
        "{}",
        v.iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

fn bubble_sort(n: usize, v_orig: &Vec<Card>) -> Vec<Card> {
    let mut v = v_orig.clone();

    for i in 0..n {
        for j in (i + 1..n).rev() {
            if v[j].value < v[j - 1].value {
                v.swap(j - 1, j);
            }
        }
    }

    v
}

fn selection_sort(n: usize, v_orig: &Vec<Card>) -> Vec<Card> {
    let mut v = v_orig.clone();

    for i in 0..n {
        let mut min_i = i;
        for j in i..v.len() {
            if v[j].value < v[min_i].value {
                min_i = j;
            }
        }
        v.swap(i, min_i);
    }
    v
}

fn is_stable(v1: &Vec<Card>, v2: &Vec<Card>) -> bool {
    v1.iter().zip(v2.iter()).all(|(i, j)| i.suit == j.suit)
}

fn main() {
    let n = scan!(usize);
    let v = scan!(String ;;)
        .iter()
        .map(|s| Card {
            suit: s[0..1].to_string(),
            value: s[1..2].parse::<usize>().expect("Expected an integer"),
        })
        .collect::<Vec<_>>();

    let v_bubble: Vec<Card> = bubble_sort(n, &v);
    let v_selection: Vec<Card> = selection_sort(n, &v);

    print_result(&v_bubble);
    // Bubble sort is always stable
    println!("Stable");

    print_result(&v_selection);
    if is_stable(&v_selection, &v_bubble) {
        println!("Stable");
    } else {
        println!("Not stable");
    }
}
