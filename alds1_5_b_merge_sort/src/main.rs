// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_5_B

fn merge(v: &mut Vec<i32>, left: usize, mid: usize, right: usize) -> usize{
    let n1: usize = mid - left;
    let n2: usize = right - mid;
    let sentinel: i32 = 2000000000;
    let mut l = v[left..(left + n1)].to_vec();
    let mut r = v[mid..(mid + n2)].to_vec();
    l.push(sentinel);
    r.push(sentinel);

    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut cnt: usize = 0;
    for k in left..right {
        cnt += 1;
        if l[i] <= r[j] {
            v[k] = l[i];
            i += 1;
        } else {
            v[k] = r[j];
            j += 1;
        }
    }
    cnt
}

fn merge_sort(v: &mut Vec<i32>, n: usize, left: usize, right: usize) -> usize {
    let mut c1 = 0;
    let mut c2 = 0;
    let mut c3 = 0;

    if left + 1 < right {
        let mid: usize = (left + right) / 2;
        c1 = merge_sort(v, n, left, mid);
        c2 = merge_sort(v, n, mid, right);
        c3 = merge(v, left, mid, right);
    }
    c1 + c2 + c3
}

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse().unwrap();

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut v: Vec<i32> = buffer
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let cnt = merge_sort(&mut v, n, 0, n);

    for (i, val) in v.iter().enumerate() {
        if i > 0 { print!(" "); }
        print!("{}", val);
    }
    print!("\n");

    println!("{}", cnt);
}
