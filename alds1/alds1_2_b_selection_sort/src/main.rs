// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_2_B

fn selection_sort(v: &mut Vec<usize>) -> usize {
    let mut count = 0;
    for i in 0..v.len() {
        let mut minj = i;
        for j in i..v.len() {
            if v[j] < v[minj] {
                minj = j;
            }
        }
        if i == minj { 
            continue 
        } else {
            v.swap(i, minj);
            count += 1;
        }
    }
    count 
}

fn main() {
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .unwrap();

    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .unwrap();
    let mut v: Vec<usize> = buffer
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let count = selection_sort(&mut v);
    for (i, val) in v.iter().enumerate() {
        if i > 0 { print!(" "); }
        print!("{}", val);
    }
    print!("\n");
    println!("{}", count);
}
