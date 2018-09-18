// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_6_A

fn counting_sort(a: &Vec<usize>, c: &mut Vec<usize>) -> Vec<usize> {
    let mut b: Vec<usize> = vec![0; a.len()];
    
    for &i in a.iter() {
        c[i] += 1;
    }

    for j in 1..c.len() {
        c[j] += c[j-1];
    }

    for &i in a.iter().rev() {
        b[c[i]-1] = i;
        c[i] -= 1;
    }
    b
}

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let a: Vec<usize> = buffer.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut c: Vec<usize> = vec![0; a.iter().max().unwrap()+1];
    let b = counting_sort(&a, &mut c);

    for (i, val) in b.iter().enumerate() {
        if i > 0 { print!(" "); }
        print!("{}", val);
    }
    print!("\n");
}
