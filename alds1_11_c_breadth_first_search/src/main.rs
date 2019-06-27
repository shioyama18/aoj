// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_11_C

use std::collections::VecDeque;

#[derive(Clone, PartialEq)]
enum Color {
    White,
    Gray,
    Black,
}

impl Color {
    fn new() -> Self {
        Color::White
    }
}

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).ok();
    let n = n.trim().parse::<usize>().unwrap();

    let mut m = vec![vec![0; n]; n];
    for _ in 0..n {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).ok();
        let v: Vec<usize> = buffer.split_whitespace().map(|x| x.parse().unwrap()).collect();
        
        let node = v[0] - 1;
        let degree = v[1];
        if degree == 0 {
            continue;
        }
        for j in 0..degree {
            m[node][v[2+j]-1] = 1;
        }
    }

    let mut color: Vec<Color> = vec![Color::new(); n];
    let mut d: Vec<isize> = vec![-1; n];    
    let mut q = VecDeque::new();

    // start from 0
    color[0] = Color::Gray;
    d[0] = 0;
    q.push_back(0);

    while let Some(u) = q.pop_front() {
        for v in 0..m.len() {
            if m[u][v] == 1 && color[v] == Color::White {
                color[v] = Color::Gray;
                d[v] = d[u] + 1;
                q.push_back(v);
            }
        }
        color[u] = Color::Black;
    }

    for u in 0..n {
        println!("{} {}", u+1, d[u]);
    }

}
