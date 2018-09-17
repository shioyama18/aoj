// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_12_A

const MAX_NODES: usize = 100;

#[derive(Clone)]
enum Color {
    White,
    Gray,
    Black,
}

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).ok();
    let n = n.trim().parse::<usize>().unwrap();

    let mut g = Vec::new();

    for _ in 0..n {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).ok();
        let buffer: Vec<isize> = buffer
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        g.push(buffer);
    }
    
    let mut color = vec![Color::White; n];
    let mut d = vec![std::isize::MAX; n];
    let mut p: Vec<isize> = vec![-1; n];

    d[0] = 0;

    loop {
        let mut mincost = std::isize::MAX;
        let mut u = MAX_NODES;

        for i in 0..n {
            match color[i] {
                Color::Black => continue,
                _ => { 
                    if d[i] < mincost {
                        mincost = d[i];
                        u = i;
                    }
                }
            }
        }

        if u == MAX_NODES { break; }
        color[u] = Color::Black;

        for v in 0..n {
            match color[v] {
                Color::Black => continue,
                _ => {
                    if g[u][v] != -1 && d[v] > g[u][v] {
                        d[v] = g[u][v];
                        p[v] = u as isize;
                        color[v] = Color::Gray;
                    }
                }
            }
        }
    }

    let sum = p
        .iter()
        .enumerate()
        .filter(|(_, &x)| x != -1)
        .fold(0, |acc, (i, &x)| acc + g[i][x as usize]);

    println!("{}", sum);

}
