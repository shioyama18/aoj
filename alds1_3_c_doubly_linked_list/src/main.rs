// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_3_C

use std::collections::VecDeque;

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n: u32 = n.trim().parse().unwrap();
    let mut v: VecDeque<u32> = VecDeque::new();

    for _ in 0..n {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let input: Vec<&str> = buffer.split_whitespace().collect();
        match input[0] {
            "insert" => v.push_front(input[1].parse().unwrap()),
            "delete" => {
                let x: u32 = input[1].parse().unwrap();
                let index = v.iter().position(|&val| x == val).unwrap_or(v.len());
                let mut tail = v.split_off(index);
                tail.pop_front();
                v.append(&mut tail);
            }
            "deleteFirst" => { v.pop_front(); },
            "deleteLast" => { v.pop_back(); },
            _ => panic!("Invalid input"),
        };
    }
    for (i, val) in v.iter().enumerate() {
        if i > 0 { print!(" "); }
        print!("{}", val);
    }
    print!("\n");
}
