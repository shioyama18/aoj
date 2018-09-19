// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_3_A

fn main() {
    let mut s: Vec<i32> = Vec::new();

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    for i in buffer.split_whitespace() {
        match i {
            "+" => {
                let a = s.pop().unwrap() as i32;
                let b = s.pop().unwrap() as i32;
                s.push(a + b);
            },
            "-" => {
                let a = s.pop().unwrap() as i32;
                let b = s.pop().unwrap() as i32;
                s.push(b - a);
            },
            "*" => {
                let a = s.pop().unwrap() as i32;
                let b = s.pop().unwrap() as i32;
                s.push(a * b);
            },
            n @ _  => s.push(n.parse().unwrap()),
        }
    }
    println!("{}", s.pop().unwrap());
}
