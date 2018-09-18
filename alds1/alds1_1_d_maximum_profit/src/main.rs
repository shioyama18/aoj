// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_1_D

use std::io;
use std::cmp::{min, max};

fn read() -> isize {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    n.trim().parse::<isize>().unwrap()
}

fn main() {
    let n = read();
    let r0 = read();
    let r1 = read();
    let mut minv: isize = min(r0, r1);
    let mut maxv: isize = r1 - r0;

    for _ in 2..n {
        let r = read();
        maxv = max(maxv, r - minv);
        minv = min(minv, r);
    }
    println!("{}", maxv);
}
