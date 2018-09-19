// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_2_D

fn insertion_sort(a: &mut Vec<u32>, g: usize) -> usize {
    let mut cnt = 0;
    let n = a.len();
    for i in g..n {
        let v = a[i];
        let mut j = i;

        while j >= g && a[j-g] > v {
            a[j] = a[j-g];
            j -= g;
            cnt += 1;
        }
        a[j] = v;
    }
    cnt
}

fn shell_sort(v: &mut Vec<u32>) -> (usize, Vec<usize>) {
    let mut cnt = 0;
    let mut g: Vec<usize> = Vec::new();
    let mut h = 1 as usize;
    while h <= v.len() {
        g.push(h);
        h = 3*h + 1;
    }
    
    for &i in g.iter().rev() {
        cnt += insertion_sort(v, i);
    }
    
    (cnt, g)
}

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();
    
    let mut v = Vec::new();
    for _ in 0..n {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let i: u32 = buffer.trim().parse().unwrap();
        v.push(i);
    }
    
    let (cnt, g) = shell_sort(&mut v);
    println!("{}", g.len());
    for (i, val) in g.iter().rev().enumerate() {
        if i > 0 { print!(" "); }
        print!("{}", val);
    }
    print!("\n");
    println!("{}", cnt);
    for val in v {
        println!("{}", val);
    }
}
