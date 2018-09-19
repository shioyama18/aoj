// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_4_A

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    //n.trim().parse().unwrap();
    
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let s: Vec<usize> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut q = String::new();
    std::io::stdin().read_line(&mut q).unwrap();
    //q.trim().parse().unwrap();
    
    let mut t = String::new();
    std::io::stdin().read_line(&mut t).unwrap();
    let t: Vec<usize> = t.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut sum: usize = 0;
    for i in &t {
        for j in &s {
            if i == j { 
                sum += 1; 
                break;
            }
        }
    }
    println!("{}", sum);
}
   
