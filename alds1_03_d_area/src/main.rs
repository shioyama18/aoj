// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_3_D

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

struct Pair(usize, usize);

fn main() {
    let mut s1: Vec<usize> = Vec::new();
    let mut s2: Vec<Pair> = Vec::new();
    let mut sum = 0;

    let buffer = scan!(String);

    for (i, c) in buffer.chars().enumerate() {
        if c == '\\' {
            s1.push(i);
        } else if c == '/' && s1.len() > 0 {
            let j = s1.pop().unwrap();
            sum += i - j;
            let mut a = i - j;
            while s2.len() > 0 && s2.last().unwrap().0 > j {
                a += s2.pop().unwrap().1;
            }
            s2.push(Pair(j, a));
        }
    }

    let mut answer: Vec<usize> = Vec::new();
    while s2.len() > 0 {
        answer.push(s2.pop().unwrap().1);
    }
    println!("{}", sum);
    println!(
        "{} {}",
        answer.len(),
        answer
            .iter()
            .rev()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
