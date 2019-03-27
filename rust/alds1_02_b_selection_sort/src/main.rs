// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_2_B

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
    let mut v = scan!(usize ;;);
    let mut cnt = 0;

    for i in 0..n {
        let mut min_i = i;

        for j in i..n {
            if v[j] < v[min_i] {
                min_i = j;
            }
        }

        if i != min_i {
            cnt += 1;
            v.swap(i, min_i);
        }
    }

    println!(
        "{}",
        v.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
    println!("{}", cnt);
}
