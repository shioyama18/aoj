// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_1_A

macro_rules! get {
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
                    get!($t)
        ).collect::<Vec<_>>()
    };
    ($($t:ty),*; $n:expr) => {
        (0..$n).map(|_|
                    get!($($t),*)
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
        (0..$n).map(|_| get!($t ;;)).collect::<Vec<_>>()
    };
}

struct DisjointSet {
    rank: Vec<usize>,
    p: Vec<Option<usize>>,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        DisjointSet { 
            rank: vec![0; n], 
            p: vec![None; n],
        }
    }
 
    fn unite(&mut self, x: usize, y: usize) {
        let nx = self.find(x);
        let ny = self.find(y);
        if nx == ny { return; }

        let (a, b) = if self.rank[nx] < self.rank[ny] {
            (nx, ny)
        } else {
            (ny, nx)
        };
        self.p[a] = Some(b);
        self.rank[b] += self.rank[a];
    }

    fn find(&mut self, x: usize) -> usize {
        match self.p[x] {
            Some(y) => {
                let res = self.find(y);
                self.p[x] = Some(res);
                res
            },
            None => x,
        }
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

}

fn main() {
    let (n, q) = get!(usize, usize);
    let mut ds = DisjointSet::new(n);

    for _ in 0..q {
        let (com, x, y) = get!(usize, usize, usize);
        if com == 0 {
            ds.unite(x, y);
        } else {
            println!("{}", if ds.same(x, y) {1} else {0});
        }
    }
}
