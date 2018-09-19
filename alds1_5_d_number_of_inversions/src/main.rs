// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_5_D

fn merge(a: &mut Vec<usize>, n: usize, left: usize, mid: usize, right: usize) -> usize {
    let n1 = mid - left;
    let n2 = right - mid;
    let mut l = a[left..(left+n1)].to_vec();
    let mut r = a[mid..(mid+n2)].to_vec();
    l.push(2000000000);
    r.push(2000000000);
    
    let mut cnt = 0;
    let mut i = 0;
    let mut j = 0;

    for k in left..right {
        if l[i] <= r[j] {
            a[k] = l[i];
            i += 1;
        } else {
            a[k] = r[j];
            j += 1;
            cnt += n1 - i;
        }
    }
    cnt
}

fn merge_sort(a: &mut Vec<usize>, n: usize, left: usize, right: usize) -> usize {
    let mut c1 = 0;
    let mut c2 = 0;
    let mut c3 = 0;

    if left + 1 < right {
        let mid: usize = (left + right) / 2;
        c1 = merge_sort(a, n, left, mid);
        c2 = merge_sort(a, n, mid, right);
        c3 = merge(a, n, left, mid, right);
    }
    c1 + c2 + c3
}

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<usize>().unwrap();

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut a: Vec<usize> = buffer.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let cnt = merge_sort(&mut a, n, 0, n);
    println!("{}", cnt);
}
