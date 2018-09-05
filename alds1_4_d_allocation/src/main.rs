// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_4_D

fn check(mid: usize, k: usize, w: &Vec<usize>) -> usize {
    // Return the total number of packages that 
    // k tracks can carry, where each track can carry mid
    let n: usize = w.len();
    let mut i: usize = 0;
    for _ in 0..k {
        let mut s: usize = 0;
        while s + w[i] <= mid {
            s += w[i];
            i += 1;
            if i == n {
               return n
            }
        }
    }
    i
}

fn solve(k: usize, w: &Vec<usize>) -> usize {
    // Do binary search to find P
    let n: usize = w.len();
    let mut min: usize = 0;
    let mut max: usize = 100000 * 10000;
    let mut mid: usize;
    while max - min > 1 {
        mid = (max + min) / 2;
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
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input: Vec<usize> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let n: usize = input[0];
    let k: usize = input[1];

    let mut w = Vec::new();
    for _ in 0..n {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let buffer: usize = buffer.trim().parse().unwrap();
        w.push(buffer);
    }

    let ans = solve(k, &w);
    println!("{}", ans);
}
