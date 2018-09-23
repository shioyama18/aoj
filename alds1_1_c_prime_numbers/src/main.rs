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

fn prime(x: usize) -> bool {
    if x < 2 { 
        return false; 
    } else if x == 2 {
        return true;
    }

    if x % 2 == 0 {
        return false;
    }

    let mut i = 3;
    while i*i <= x {
        if x % i == 0 { return false; }
        i += 2;
    }
    true
}

fn main() {
    let n = scan!(usize);
    let mut cnt = 0;
    for _ in 0..n {
        let x = scan!(usize);
        if prime(x) { cnt += 1; }
    }
    println!("{}", cnt);
}
