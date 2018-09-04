// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_4_B

use std::cmp::Ordering;

fn binary_search(s: &Vec<usize>, i: usize) -> Option<usize> {
    let n = s.len();
    let mid_index = n / 2;
    let mid = s[mid_index];

    match mid.cmp(&i) {
        Ordering::Less => binary_search(&s[(mid_index + 1)..n].to_vec(), i),
        Ordering::Equal => Some(mid as usize),
        Ordering::Greater => binary_search(&s[0..mid_index].to_vec(), i),
    }
}

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    // n.trim().parse().unwrap();
    
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let s: Vec<usize> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut q = String::new();
    std::io::stdin().read_line(&mut q).unwrap();
    // q.trim().parse().unwrap();
    
    let mut t = String::new();
    std::io::stdin().read_line(&mut t).unwrap();
    let t: Vec<usize> = t.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut sum: usize = 0;
    for &i in t.iter() {
        match binary_search(&s, i) {
            Some(_) => sum += 1,
            None => (),
        }
    }
    println!("{}", sum);
}
