// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_1_A

use std::io;

fn trace(v: &Vec<usize>) {
    for (i, val) in v.iter().enumerate() {
        if i > 0 { print!(" "); }
        print!("{}", val);
    }
    print!("\n");
}

fn insertion_sort(v: &mut Vec<usize>) {
    for i in 1..v.len() {
        let cur_val = v[i];
        let mut insert_loc = i;

        // Shift all elements greater than current value
        while insert_loc >= 1 && v[insert_loc - 1] > cur_val { 
            v[insert_loc] = v[insert_loc - 1];
            insert_loc -= 1;
        }

        // Insert current value
        v[insert_loc] = cur_val;
        trace(&v);
    }
}

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    // let n: usize = n.trim().parse().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut v: Vec<usize> = buffer.split_whitespace().map(|s| s.parse().unwrap()).collect();
    trace(&v);
    insertion_sort(&mut v);
}

