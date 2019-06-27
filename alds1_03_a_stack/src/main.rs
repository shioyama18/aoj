// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_3_A

macro_rules! scan {
    ($t:ty) => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<$t>().unwrap()
        }
    };
    ($($t:ty),*) => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                $(iter.next().unwrap().parse::<$t>().unwrap(),)*
            )
        }
    };
    ($t:ty; $n:expr) => {
        (0..$n).map(|_|
                    scan!($t)
        ).collect::<Vec<_>>()
    };
    ($($t:ty),*; $n:expr) => {
        (0..$n).map(|_|
                    scan!($($t),*)
        ).collect::<Vec<_>>()
    };
    ($t:ty ;;) => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.split_whitespace()
                .map(|t| t.parse::<$t>().unwrap())
                .collect::<Vec<_>>()
        }
    };
    ($t:ty ;; $n:expr) => {
        (0..$n).map(|_| scan!($t ;;)).collect::<Vec<_>>()
    };
}

fn eval(op: &str, stack: &mut Vec<isize>) {
    let right = stack.pop().expect("No operand");
    let left = stack.pop().expect("No operand");

    match op {
        "+" => stack.push(left + right),
        "-" => stack.push(left - right),
        "*" => stack.push(left * right),
        _ => panic!("Expected an operator"),
    }
}

fn main() {
    let mut stack = Vec::new();
    let input = scan!(String ;;);

    for i in input {
        match i.as_str() {
            "+" | "-" | "*" => eval(&i, &mut stack),
            _ => stack.push(i.parse::<isize>().expect("Expected an integer")),
        }
    }
    println!("{}", stack.pop().expect("Expected an integer"));
}
