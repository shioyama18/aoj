// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_1_A

use std::io;

fn trace(v: &Vec<usize>) {
    println!(
        "{}",
        v.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

fn insertion_sort(n: usize, input: &mut Vec<usize>) {
    for i in 1..n {
        let key = input[i];
        let mut j: isize = i as isize - 1;

        while j >= 0 && input[j as usize] > key {
            input[j as usize + 1] = input[j as usize];
            j -= 1;
        }
        input[(j + 1) as usize] = key;
        trace(input);
    }
}

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<usize>().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut input = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();

    trace(&input);
    insertion_sort(n, &mut input);
}
