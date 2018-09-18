// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_6_C

#[derive(Clone)]
struct Card {
    suit: String,
    value: usize,
}

fn merge(v: &mut Vec<Card>, left: usize, mid: usize, right: usize) {
    let n1 = mid - left;
    let n2 = right - mid;
    let sentinel = Card { suit: String::from("Z"), value: 2000000000 };  
    let mut l = v[left..(left + n1)].to_vec();
    let mut r = v[mid..(mid + n2)].to_vec();

    l.push(sentinel.clone());
    r.push(sentinel.clone());

    let mut i: usize = 0;
    let mut j: usize = 0;

    for k in left..right {
        if l[i].value <= r[j].value {
            v[k] = l[i].clone();
            i += 1;
        } else {
            v[k] = r[j].clone();
            j += 1;
        }
    }
}

fn merge_sort(v: &mut Vec<Card>, n: usize, left: usize, right: usize) {
    if left + 1 < right {
        let mid: usize = (left + right) / 2;
        merge_sort(v, n, left, mid);
        merge_sort(v, n, mid, right);
        merge(v, left, mid, right);
    }
}

fn partition(v: &mut[Card]) -> usize {
    let n = v.len();
    let x = v.last().unwrap().clone();
    let mut i = 0;
    for j in 0..n {
        if v[j].value <= x.value {
            v.swap(i, j);
            i += 1;
        }
    }
    i - 1
}

fn quick_sort(v: &mut[Card]) {
    if v.len() >= 2 {
        let q = partition(v);
        quick_sort(&mut v[..q]);
        quick_sort(&mut v[q..]);
    }
}

fn is_stable(v1: &Vec<Card>, v2: &Vec<Card>) -> bool {
    for (i, j) in v1.iter().zip(v2.iter()) {
        if i.suit != j.suit {
            return false
        }
    }
    true
}

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<usize>().unwrap();
    
    let mut v: Vec<Card> = Vec::with_capacity(n);

    for _ in 0..n {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let buffer: Vec<&str> = buffer.split_whitespace().collect();
        v.push(Card {
            suit: buffer[0].to_string(),
            value: buffer[1].parse::<usize>().unwrap(),
        });
    }
    let mut v_merge = v.clone();

    quick_sort(&mut v[..]);
    merge_sort(&mut v_merge, n, 0, n);

    if is_stable(&v, &v_merge) {
        println!("Stable");
    } else {
        println!("Not stable");
    }

    for val in v.iter() {
        println!("{} {}", val.suit, val.value);
    }

}
