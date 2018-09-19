// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_4_C

use std::collections::HashSet;

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<u32>().unwrap();

    let mut h = HashSet::new();
    for _ in 0..n {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let input: Vec<String> = buffer.split_whitespace().map(|x| x.to_string()).collect();
        match input[0].as_str() {
            "insert" => {
                h.insert(input[1].clone());
            },
            "find" => {
                if h.contains(&input[1]) {
                    println!("yes");
                } else {
                    println!("no");
                }
            },
            _ => (),
        }
    }
}
