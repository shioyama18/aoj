// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_10_C

use std::collections::VecDeque;

fn lcs(s1: &VecDeque<char>, s2: &VecDeque<char>) -> usize {
    let m = s1.len();
    let n = s2.len();
    let mut c = vec![vec![0; n]; m];
    let mut max = 0;
    
    for i in 1..m {
        for j in 1..n {
            c[i][j] = if s1[i] == s2[j] {
                c[i-1][j-1] + 1
            } else if c[i-1][j] >= c[i][j-1] {
                c[i-1][j]
            } else {
                c[i][j-1]
            };

            max = if max > c[i][j] {
                max
            } else {
                c[i][j]
            };
        }
    }
    max
}

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).ok();
    let n = n.trim().parse::<usize>().unwrap();
    
    for _ in 0..n {
        let mut s1 = String::new();
        std::io::stdin().read_line(&mut s1).ok();
        let mut s1: VecDeque<char> = s1.split_whitespace()
            .map(|e| e.parse::<String>().ok().unwrap())
            .collect::<String>()
            .chars()
            .collect();
        s1.push_front(' ');

        let mut s2 = String::new();
        std::io::stdin().read_line(&mut s2).ok();
        let mut s2: VecDeque<char> = s2.split_whitespace()
            .map(|e| e.parse::<String>().ok().unwrap())
            .collect::<String>()
            .chars()
            .collect();
        s2.push_front(' ');

        println!("{}", lcs(&s1, &s2));
    }
}
