// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_2_A
fn bubble_sort(v: &mut Vec<usize>) -> usize {
    let mut flag = true;
    let mut cnt = 0;
    let n = v.len();

    while flag {
        flag = false;
        for j in (1..n).rev() {
            if v[j] < v[j-1] {
                v.swap(j, j-1);
                flag = true;
                cnt += 1;
            }
        }
    }
    cnt            
}

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut v: Vec<usize> = buffer.split_whitespace().map(|s| s.parse().unwrap()).collect();

    let count = bubble_sort(&mut v);

    for (i, val) in v.iter().enumerate() {
        if i > 0 { print!(" "); }
        print!("{}", val);
    }
    print!("\n");
    println!("{}", count);
}
