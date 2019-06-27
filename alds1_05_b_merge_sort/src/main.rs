// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_5_B

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

fn merge(v: &mut Vec<isize>, left: usize, mid: usize, right: usize) -> usize {
    let n1: usize = mid - left;
    let n2: usize = right - mid;
    let sentinel = std::isize::MAX;

    let mut l = v[left..(left + n1)].to_vec();
    let mut r = v[mid..(mid + n2)].to_vec();
    l.push(sentinel);
    r.push(sentinel);

    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut cnt: usize = 0;
    for k in left..right {
        cnt += 1;
        if l[i] <= r[j] {
            v[k] = l[i];
            i += 1;
        } else {
            v[k] = r[j];
            j += 1;
        }
    }
    cnt
}

fn merge_sort(v: &mut Vec<isize>, n: usize, left: usize, right: usize) -> usize {
    let mut c1 = 0;
    let mut c2 = 0;
    let mut c3 = 0;

    if left + 1 < right {
        let mid: usize = (left + right) / 2;
        c1 = merge_sort(v, n, left, mid);
        c2 = merge_sort(v, n, mid, right);
        c3 = merge(v, left, mid, right);
    }
    c1 + c2 + c3
}

fn main() {
    let n = scan!(usize);
    let mut v = scan!(isize ;;);
    let cnt = merge_sort(&mut v, n, 0, n);

    println!(
        "{}",
        v.iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );

    println!("{}", cnt);
}
