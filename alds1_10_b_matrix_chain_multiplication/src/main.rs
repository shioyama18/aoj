// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_10_B

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).ok();
    let n = n.trim().parse::<usize>().unwrap();
    
    let mut p: Vec<Vec<usize>> = Vec::new();
    for _ in 0..n {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).ok();
        let buffer: Vec<usize> = buffer
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        p.push(buffer);
    }

    let mut m = vec![vec![0; n]; n];

    for l in 1..n {
        for i in 0..(n-l) {
            let j = i + l;
            if l == 1 {
                m[i][j] = p[i][0] * p[i][1] * p[j][1];
            } else {
                m[i][j] = (i..j)
                    .map(|k| {
                        m[i][k] + m[k+1][j]
                            + p[i][0] * p[k+1][0] * p[j][1]
                    })
                    .fold(std::usize::MAX, |min, x| std::cmp::min(min, x));
            }
        }
    }
    println!("{}", m[0][n-1]);
}
