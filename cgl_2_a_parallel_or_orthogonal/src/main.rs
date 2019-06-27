// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=CGL_2_A

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

use std::ops::Sub;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn dot(p1: Self, p2: Self) -> f64 {
        p1.x * p2.x + p1.y * p2.y
    }

    fn cross(p1: Self, p2: Self) -> f64 {
        p1.x * p2.y - p1.y * p2.x
    }
}

impl Sub for Point {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Debug)]
struct Segment {
    p1: Point,
    p2: Point,
}

impl Segment {
    fn is_orthogonal(s1: &Self, s2: &Self) -> bool {
        Point::dot(s1.p2 - s1.p1, s2.p2 - s2.p1) == 0.0
    }

    fn is_parallel(s1: &Self, s2: &Self) -> bool {
        Point::cross(s1.p2 - s1.p1, s2.p2 - s2.p1) == 0.0
    }

}


fn main() {
    let q = scan!(usize);
    for _ in 0..q {
        let buffer = scan!(f64;;);
        let s = buffer
            .chunks(4)
            .map(|chunk| 
                 Segment {
                    p1: Point { x: chunk[0], y: chunk[1] },
                    p2: Point { x: chunk[2], y: chunk[3] },
                 }
            )
            .collect::<Vec<_>>();;

        if Segment::is_parallel(&s[0], &s[1]) {
            println!("2");
        } else if Segment::is_orthogonal(&s[0], &s[1]) {
            println!("1");
        } else {
            println!("0");
        }
    }
}
