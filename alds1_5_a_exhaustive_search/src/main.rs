// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_5_A

fn solve(i: usize, m: i32, a: &Vec<i32>) -> bool {
    if m == 0 { 
        true 
    } else if i >= a.len() { 
        false 
    } else {
        solve(i + 1, m, a) || solve(i + 1, m - a[i], a)
    }
}

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let mut a = String::new();
    std::io::stdin().read_line(&mut a).unwrap();
    let a: Vec<i32> = a.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut q = String::new();
    std::io::stdin().read_line(&mut q).unwrap();
    let mut m = String::new();
    std::io::stdin().read_line(&mut m).unwrap();
    let m: Vec<i32> = m.split_whitespace().map(|x| x.parse().unwrap()).collect();
    
    for &M in m.iter() {
        if solve(0, M, &a) {
            println!("yes");
        } else {
            println!("no");
        }
    }
}
