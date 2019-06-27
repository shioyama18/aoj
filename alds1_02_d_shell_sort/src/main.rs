// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_2_D

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

fn insertion_sort(a: &mut Vec<usize>, g: usize) -> usize {
    let mut cnt = 0;
    let n = a.len();

    for i in g..n {
        let v = a[i];
        let mut j = i;

        while j >= g && a[j - g] > v {
            a[j] = a[j - g];
            j -= g;
            cnt += 1;
        }
        a[j] = v;
    }

    cnt
}

fn shell_sort(v: &mut Vec<usize>) -> (usize, Vec<usize>) {
    let mut cnt = 0;
    let mut g = Vec::new();
    let mut h = 1;
    while h <= v.len() {
        g.push(h);
        h = 3 * h + 1;
    }

    for &i in g.iter().rev() {
        cnt += insertion_sort(v, i);
    }

    (cnt, g)
}

fn main() {
    let n = scan!(usize);
    let mut v = scan!(usize; n);

    let (cnt, g) = shell_sort(&mut v);

    println!("{}", g.len());
    println!(
        "{}",
        g.iter()
            .rev()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );

    println!("{}", cnt);
    v.iter().for_each(|val| println!("{}", val));
}
