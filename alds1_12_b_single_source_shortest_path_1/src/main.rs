// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_12_B

const MAX_NODES: usize = 100;

#[derive(Clone, Debug)]
enum Color {
    White,
    Gray,
    Black,
}

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).ok();
    let n = n.trim().parse::<usize>().unwrap();

    let mut g = vec![vec![std::usize::MAX; n]; n];
    
    for _ in 0..n {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).ok();
        let buffer: Vec<usize> = buffer
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();  
        let v1 = buffer[0];
        let degree = buffer[1];
        for i in (0..degree*2).step_by(2) {
            let v2 = buffer[2+i];
            let cost = buffer[3+i];
            g[v1][v2] = cost;
        }
    }

    let mut color = vec![Color::White; n];
    let mut d = vec![std::usize::MAX; n];

    d[0] = 0;
    color[0] = Color::Gray;

    loop {
        let mut mincost = std::usize::MAX;
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
                    if g[u][v] != std::usize::MAX && d[v] > d[u] + g[u][v] {
                        d[v] = d[u] + g[u][v];
                        color[v] = Color::Gray;
                    }
                }
            }
        }
    }

    for i in 0..n {
        if d[i] == std::usize::MAX { 
            println!("{} -1", i); 
        } else {
            println!("{} {}", i, d[i]);
        }
    }

}
