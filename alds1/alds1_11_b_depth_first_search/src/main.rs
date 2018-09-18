// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_11_B

#[derive(Clone)]
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

fn dfs(u: usize, m: &Vec<Vec<usize>>, d: &mut Vec<usize>, f: &mut Vec<usize>, color: &mut Vec<Color>, time: &mut usize) {
    color[u] = Color::Gray;
    *time += 1;
    d[u] = *time;
    
    for v in 0..m.len() {
        if m[u][v] == 0 { continue; }
        if let Color::White = color[v] {
            dfs(v, m, d, f, color, time);
        }
    }

    color[u] = Color::Black;
    *time += 1;
    f[u] = *time;
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

    let mut d: Vec<usize> = vec![0; n];    
    let mut f: Vec<usize> = vec![0; n];

    let mut color: Vec<Color> = vec![Color::new(); n];
    let mut time: usize = 0;
    
    for u in 0..n {
        if let Color::White = color[u] {
            dfs(u, &m, &mut d, &mut f, &mut color, &mut time);
        }
    }

    for u in 0..n {
        println!("{} {} {}", u+1, d[u], f[u]);
    }

}
