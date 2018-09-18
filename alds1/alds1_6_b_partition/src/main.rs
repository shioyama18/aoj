// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_6_B

fn partition(v: &mut Vec<usize>, n: usize) -> usize {
    let x = v[n-1];
    let mut i = 0;
    for j in 0..n {
        if v[j] <= x {
            v.swap(i, j);
            i += 1;
        }
    }
    i - 1
}

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();
    
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut v: Vec<usize> = buffer.split_whitespace().map(|x| x.parse().unwrap()).collect();
    
    let index = partition(&mut v, n);

    for (i, val) in v.iter().enumerate() {
        if i > 0 { print!(" "); }
        if i == index { 
            print!("[{}]", val); 
        } else {
            print!("{}", val);
        }
    }
    print!("\n");
}
