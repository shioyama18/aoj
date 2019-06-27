// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_11_A

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).ok();
    let n = n.trim().parse::<usize>().unwrap();

    let mut g = vec![vec![0; n]; n];
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
            g[node][v[2+j]-1] = 1;
        }
    }
    for i in g.iter() {
        for (index, val) in i.iter().enumerate() {
            if index > 0 { print!{" "}; }
            print!("{}", val);
        }
        print!("\n");
    }
}
