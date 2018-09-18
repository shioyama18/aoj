// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_3_D

struct Pair(usize, usize);

fn main() {
    let mut s1: Vec<usize> = Vec::new();
    let mut s2: Vec<Pair> = Vec::new();
    let mut sum = 0;

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    for (i, c) in buffer.chars().enumerate() {
        if c == '\\' { 
            s1.push(i); 
        } else if c == '/' && s1.len() > 0 { 
            let j = s1.pop().unwrap();
            sum += i - j;
            let mut a = i - j;
            while s2.len() > 0 && s2.last().unwrap().0 > j {
                a += s2.pop().unwrap().1; 
            }
            s2.push(Pair(j, a));
        }
    }

    let mut answer: Vec<usize> = Vec::new();
    while s2.len() > 0 {
        answer.push(s2.pop().unwrap().1);
    }
    println!("{}", sum);
    print!("{}", answer.len());
    for i in answer.iter().rev() {
        print!(" {}", i);
    }
    print!("\n");
}
