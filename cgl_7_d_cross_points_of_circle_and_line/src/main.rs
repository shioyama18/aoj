// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=CGL_7_D

#![allow(dead_code)]

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

enum Rotation {
    CounterClockwise,
    Clockwise,
    OnlineBack,
    OnlineFront,
    OnSegment,
}

impl Rotation {
    fn translate(&self) -> isize {
        match self {
            Rotation::CounterClockwise => 1,
            Rotation::Clockwise => -1,
            Rotation::OnSegment => 0,
            Rotation::OnlineBack => 2,
            Rotation::OnlineFront => -2,
        }
    }
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

    fn distance(p1: Self, p2: Self) -> f64 {
        ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
    }

    fn ccw(p1: Self, p2: Self, p3: Point) -> Rotation {
        let a = p2 - p1;
        let b = p3 - p1;
        if Point::cross(a, b) > 0.0 {
            Rotation::CounterClockwise
        } else if Point::cross(a, b) < 0.0 {
            Rotation::Clockwise
        } else if Point::dot(a, b) < 0.0 {
            Rotation::OnlineBack
        } else if a.norm() < b.norm() {
            Rotation::OnlineFront
        } else {
            Rotation::OnSegment
        }
    }

    fn intersect(p1: Self, p2: Self, p3: Self, p4: Self) -> bool {
        let a = Point::ccw(p1, p2, p3).translate();
        let b = Point::ccw(p1, p2, p4).translate();
        let c = Point::ccw(p3, p4, p1).translate();
        let d = Point::ccw(p3, p4, p2).translate();

        a * b <= 0 && c * d <= 0
    }
}

type Vector = Point;

use std::ops::{Add, Sub, Mul, Div};

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

impl Div<f64> for Point {
    type Output = Self;
    fn div(self, k: f64) -> Self {
        Point {
            x: self.x / k,
            y: self.y / k,
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

    fn reflect(&self, p: Point) -> Point {
        p + (self.project(p) - p) * 2.0
    }

    fn intersect(&self, other: &Self) -> bool {
        Point::intersect(self.p1, self.p2, other.p1, other.p2)
    }

    fn distance_segement_point(&self, p: Point) -> f64 {
        if Point::dot(self.p2 - self.p1, p - self.p1) < 0.0 {
            Point::distance(p, self.p1)
        } else if Point::dot(self.p1 - self.p2, p - self.p2) < 0.0 {
            Point::distance(p, self.p2)            
        } else {
            (Point::cross(self.p2 - self.p1, p - self.p1) /
                Point::distance(self.p2, self.p1)).abs()
        }
    }

    fn distance(&self, other: &Self) -> f64 {
        if self.intersect(other) { 
            0.0 
        } else {
            self.distance_segement_point(other.p1)
                .min(self.distance_segement_point(other.p2))
                .min(other.distance_segement_point(self.p1))
                .min(other.distance_segement_point(self.p2))
        }
    }

    fn cross_point(&self, other: &Self) -> Point {
        let base = other.p2 - other.p1;
        let d1 = Point::cross(base, self.p1 - other.p1).abs();
        let d2 = Point::cross(base, self.p2 - other.p1).abs();
        let t = d1 / (d1 + d2);
        self.p1 + (self.p2 - self.p1) * t
    }
}

type Line = Segment;

struct Circle {
    c: Point,
    r: f64,
}

impl Circle {
    fn new(c: Point, r: f64) -> Self {
        Circle { c, r }
    }

    fn cross_point(&self, l: &Line) -> (Point, Point) {
        let pr = l.project(self.c);
        let e = (l.p2 - l.p1) / Point::distance(l.p2, l.p1);
        let base = (self.r * self.r - (pr - self.c).norm()).sqrt();
        (pr + e * base, pr - e * base)
    }
}    

fn main() {
    let (x, y, r) = scan!(f64, f64, f64);
    let c = Circle::new(Point{ x, y }, r);

    let q = scan!(usize);

    for _ in 0..q {
        let buffer = scan!(f64;;);
        let s = buffer
            .chunks(4)
            .map(|chunk| 
                 Line {
                     p1: Point { x: chunk[0], y: chunk[1] },
                     p2: Point { x: chunk[2], y: chunk[3] },
                 }
            )
            .collect::<Vec<_>>();;
        let line = s[0].clone();
        let (p1, p2) = c.cross_point(&line);
        
        if p1.x < p2.x {
            println!("{} {} {} {}", p1.x, p1.y, p2.x, p2.y);
        } else if p1.x > p2.x {
            println!("{} {} {} {}", p2.x, p2.y, p1.x, p1.y);
        } else {
            if p1.y < p2.y {
                println!("{} {} {} {}", p1.x, p1.y, p2.x, p2.y);
            } else if p1.y > p2.y {
                println!("{} {} {} {}", p2.x, p2.y, p1.x, p1.y);
            } else {
                println!("{} {} {} {}", p1.x, p1.y, p2.x, p2.y);
            }
        }
    }
}
