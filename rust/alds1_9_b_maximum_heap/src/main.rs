// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_9_B

use std::collections::VecDeque;

fn max_heapify(a: &mut VecDeque<isize>, i: usize) {
    let l = i*2;
    let r = i*2+1;
    let mut largest: usize;

    if l <= a.len()-1 && a[l] > a[i] {
        largest = l;
    } else {
        largest = i;
    }

    if r <= a.len()-1 && a[r] > a[largest] {
        largest = r;
    }
    
    if largest != i {
        a.swap(i, largest);
        max_heapify(a, largest);
    }
}

fn build_max_heap(a: &mut VecDeque<isize>, n: usize) {
    for i in (1..n).rev() {
        max_heapify(a, i);
    }
}

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).ok();
    let n = n.trim().parse::<usize>().unwrap();

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).ok();
    let mut buffer: VecDeque<isize> = buffer.split_whitespace().map(|x| x.parse().unwrap()).collect();
    buffer.push_front(0);
    build_max_heap(&mut buffer, n / 2 + 1);

    for (i, val) in buffer.iter().enumerate() {
        if i == 0 { continue; }
        print!(" {}", val);
    }
    print!("\n");
}
