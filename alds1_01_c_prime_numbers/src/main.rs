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

fn main() -> Result<(), Box<std::error::Error>> {
    let n = scan!(usize);

    let mut counter = 0;
    let mut max = 2;
    let mut primes = Vec::new();

    for _ in 0..n {
        // Read number
        let x = scan!(usize);

        // Add all prime numbers upto and including x to list of primes
        'outer: while max <= x {
            for p in primes.iter() {
                if max % p == 0 {
                    max += 1;
                    continue 'outer;
                }
            }

            primes.push(max);
            max += 1;
        }

        // If x is in primes, increment the counter
        if primes.contains(&x) {
            counter += 1;
        }
    }

    println!("{}", counter);

    Ok(())
}
