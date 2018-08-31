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
    let mut insert_loc: isize; // location to insert current value
    let mut cur_val: usize;    // current value

    for i in 1..v.len() {
        cur_val = v[i];
        insert_loc = (i - 1) as isize;

        // Shift all elements greater than current value
        while insert_loc >= 0 && v[insert_loc as usize] > cur_val { 
            v[(insert_loc + 1) as usize] = v[insert_loc as usize];
            insert_loc = (insert_loc - 1) as isize;
        }

        // Insert Value
        v[(insert_loc + 1) as usize] = cur_val;
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

