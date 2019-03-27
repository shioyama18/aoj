// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_1_C

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

fn main() {
    let n = scan!(usize);

    let mut counter = 0;
    let mut i = 2;
    let mut primes = Vec::new();

    for _ in 0..n {
        let x = scan!(usize);

        while i * i <= x {
            if primes.clone().iter().all(|p| i % p != 0) {
                primes.push(i);
            }
            i += 1;
        }

        if primes.contains(&x) {
            counter += 1;
        } else {
            if primes.clone().iter().all(|p| x % p != 0) {
                primes.push(x);
                counter += 1;
            }
        }
    }

    println!("{}", counter);
}
