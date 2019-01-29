// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_5_A

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

fn solve(i: usize, m: isize, a: &[isize]) -> bool {
    if m == 0 {
        true
    } else if i >= a.len() {
        false
    } else {
        solve(i + 1, m, a) || solve(i + 1, m - a[i], a)
    }
}

fn main() {
    let _n = scan!(usize);
    let a = scan!(isize;;);

    let _q = scan!(usize);
    let ms = scan!(isize;;);

    for m in ms {
        if solve(0, m, &a) {
            println!("yes");
        } else {
            println!("no");
        }
    }
}
