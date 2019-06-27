// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_6_D

const VMAX: usize = 10000;

fn solve(a: &mut Vec<usize>, s: usize) -> usize {
    let mut ans = 0;
    let mut b: Vec<usize> = a.clone();
    let mut v: Vec<bool> = vec![false; a.len()];
    b.sort();

    let mut t: Vec<usize> = vec![0; VMAX+1];
    for i in 0..b.len() {
        t[b[i]] = i;
    }

    for i in 0..b.len() {
        if v[i] { continue; }
        let mut cur = i;
        #[allow(non_snake_case)]
        let mut S = 0;
        let mut m = VMAX;
        let mut an = 0;
        loop {
            v[cur] = true;
            an += 1;
            let x = a[cur];
            m = std::cmp::min(m, x);
            S += x;
            cur = t[x];
            if v[cur] { break; }
        }
        ans += std::cmp::min(S as isize + (an as isize - 2) * m as isize, (m + S + (an + 1) * s) as isize) as usize;
    }
    ans
}

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    // let n = n.trim().parse::<usize>().unwrap();

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut a: Vec<usize> = buffer.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let &s = a.iter().min().unwrap();
    let ans = solve(&mut a, s);
    println!("{}", ans);
}
