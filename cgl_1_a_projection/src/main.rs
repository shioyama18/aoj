// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=CGL_1_A

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


#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn norm(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    fn dot(p1: Self, p2: Self) -> f64 {
        p1.x * p2.x + p1.y * p2.y
    }

    fn cross(p1: Self, p2: Self) -> f64 {
        p1.x * p2.y - p1.y * p2.x
    }
}

use std::ops::{Add, Sub, Mul};

impl Add for Point {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
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

impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, k: f64) -> Self {
        Point {
            x: self.x * k,
            y: self.y * k,
        }
    }
}

#[derive(Debug, Clone)]
struct Segment {
    p1: Point,
    p2: Point,
}

impl Segment {
    fn project(&self, p: Point) -> Point {
        let base = self.p2 - self.p1;
        let r = Point::dot(p - self.p1, base) / base.norm();
        self.p1 + base * r
    }
}

fn main() {
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
    let s = s[0].clone();

    let q = scan!(usize);
    for _ in 0..q {
        let (x, y) = scan!(f64, f64);
        let p = s.project(Point { x, y });
        println!("{} {}", p.x, p.y);
    }
}
