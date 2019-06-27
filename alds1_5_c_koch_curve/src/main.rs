// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_5_C

struct Point {
    x: f64,
    y: f64,
}

fn koch(n: usize, a: &Point, b: &Point) {
    if n == 0 { return }
    
    let th: f64 = std::f64::consts::PI * 60.0 / 180.0;

    let s = Point { 
        x: (2.0 * a.x + 1.0 * b.x) / 3.0, 
        y: (2.0 * a.y + 1.0 * b.y) / 3.0
    };
    let t = Point { 
        x: (1.0 * a.x + 2.0 * b.x) / 3.0, 
        y: (1.0 * a.y + 2.0 * b.y) / 3.0 
    };
    let u = Point { 
        x: (t.x - s.x) * th.cos() - (t.y - s.y) * th.sin() + s.x, 
        y: (t.x - s.x) * th.sin() + (t.y - s.y) * th.cos() + s.y 
    };

    koch(n - 1, a, &s);
    println!("{:.8} {:.8}", s.x, s.y);    
    koch(n - 1, &s, &u);
    println!("{:.8} {:.8}", u.x, u.y);    
    koch(n - 1, &u, &t);
    println!("{:.8} {:.8}", t.x, t.y);    
    koch(n - 1, &t, b);
}    

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<usize>().unwrap();

    let a = Point { x: 0.0, y: 0.0 };
    let b = Point { x: 100.0, y: 0.0 };

    println!("{:.8} {:.8}", a.x, a.y);
    koch(n, &a, &b);
    println!("{:.8} {:.8}", b.x, b.y);    
}
