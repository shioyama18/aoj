// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_2_A

fn swap_count(v: &mut Vec<usize>, i: usize, j: usize, count: usize) -> usize {
    v.swap(i, j);
    count + 1
}

fn bubble_sort(v: &mut Vec<usize>) -> usize {
    let mut flag = true;
    let mut count = 0;
    let mut i = 0;

    while flag {
        flag = false;
        for j in (i + 1..v.len()).rev() {
            if v[j] < v[j-1] {
                flag = true;
                count = swap_count(v, j-1, j, count);
            }
        }
        i += 1;
    }
    count            
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
