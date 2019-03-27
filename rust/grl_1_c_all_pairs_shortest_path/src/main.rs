// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_1_C

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

fn floyd(ds: &mut Vec<Vec<Option<i32>>>, n: usize) {
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if let (Some(d1), Some(d2)) = (ds[i][k], ds[k][j]) {
                    if ds[i][j].map(|d| d > d1 + d2).unwrap_or(true) {
                        ds[i][j] = Some(d1 + d2);
                    }
                }
            }
        }
    }
}

fn main() {
    let (n, e) = scan!(usize, usize);
    let input = scan!(usize, usize, i32; e);
    let mut ds = vec![vec![None; n]; n];

    for i in 0..n {
        ds[i][i] = Some(0);
    }

    for (s, t, d) in input {
        ds[s][t] = Some(d);
    }

    floyd(&mut ds, n);

    if (0..n).any(|i| ds[i][i].unwrap() < 0) {
        println!("NEGATIVE CYCLE");
    } else {
        for i in 0..n {
            println!(
                "{}", 
                ds[i]
                    .iter()
                    .map(|d| 
                         d.map(|x| x.to_string())
                         .unwrap_or("INF".into()))
                    .collect::<Vec<_>>()
                    .join(" ")
            );

        }
    }
}

