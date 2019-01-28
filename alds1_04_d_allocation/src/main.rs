// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_4_D

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

fn check(mid: usize, k: usize, w: &[usize]) -> usize {
    // Return the total number of packages that
    // k tracks can carry, where each track can carry mid
    let n = w.len();
    let mut i = 0;

    for _ in 0..k {
        let mut s = 0;
        while s + w[i] <= mid {
            s += w[i];
            i += 1;
            if i == n {
                return n;
            }
        }
    }

    i
}

fn solve(k: usize, w: &[usize]) -> usize {
    // Do binary search to find P
    let n = w.len();
    let mut min = 0;
    let mut max = std::usize::MAX;

    while max - min > 1 {
        let mid = (max + min) / 2;
        let v = check(mid, k, w);
        if v >= n {
            max = mid;
        } else {
            min = mid;
        }
    }

    max
}

fn main() {
    let buffer = scan!(usize ;;);
    let n = buffer[0];
    let k = buffer[1];
    let w = scan!(usize; n);

    let p = solve(k, &w);
    println!("{}", p);
}
