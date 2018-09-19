// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_9_A

use std::collections::VecDeque;

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).ok();
    let n = n.trim().parse::<usize>().unwrap();

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).ok();
    let mut buffer: VecDeque<isize> = buffer.split_whitespace().map(|x| x.parse().unwrap()).collect();
    buffer.push_front(0);

    for i in 1..buffer.len() {
        print!("node {}: key = {}, ", i, buffer[i]);
        if i > 1 {
            print!("parent key = {}, ", buffer[i/2 as usize]);
        }
        if n >= i*2 {
            print!("left key = {}, ", buffer[i*2]);
            if n >= i*2+1 {
                print!("right key = {}, ", buffer[i*2+1]);                
            }
        }
        print!("\n");
    }
        
}
