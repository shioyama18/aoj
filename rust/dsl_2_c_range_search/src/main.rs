// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_C

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

fn make_tree(seq: &mut Vec<(usize, usize, usize)>, nodes: &mut Vec<(usize, Option<usize>, Option<usize>)>, depth: usize, l: usize, r: usize) -> Option<usize> {
    if l >= r {
        return None;
    }
    
    if depth % 2 == 0 {
        seq[l..r].sort_by_key(|t| t.0);
    } else {
        seq[l..r].sort_by_key(|t| t.1);
    }

    let mid = (l + r) / 2;    
    let t = nodes.len();
    
    nodes.push((mid, None, None));
    nodes[t].1 = make_tree(seq, nodes, depth + 1, l, mid);
    nodes[t].2 = make_tree(seq, nodes, depth + 1, mid + 1, r);
    Some(t)
}

fn find(
    depth: usize,
    node: usize,
    seq: &[(usize, usize, usize)],
    tree: &[(usize, Option<usize>, Option<usize>)],
    sx: usize,
    sy: usize,
    tx: usize,
    ty: usize,
    v: &mut Vec<usize>
) {
    let (t, l, r) = tree[node];
    let (x, y, i) = seq[t];

    if sx <= x && x <= tx && sy <= y && y <= ty {
        v.push(i);
    }
 
    if depth % 2 == 0 {
        if l.is_some() && sx <= x {
            find(depth + 1, l.unwrap(), seq, tree, sx, sy, tx, ty, v);
        }
        if r.is_some() && tx >= x {
            find(depth + 1, r.unwrap(), seq, tree, sx, sy, tx, ty, v);
        }
    } else {
        if l.is_some() && sy <= y {
            find(depth + 1, l.unwrap(), seq, tree, sx, sy, tx, ty, v);
        }
        if r.is_some() && ty >= y {
            find(depth + 1, r.unwrap(), seq, tree, sx, sy, tx, ty, v);
        }
    }
}


fn main() {
    let n = get!(usize);
    let xy = get!(usize, usize; n);
    let mut seq: Vec<(usize, usize, usize)> = xy.into_iter()
        .enumerate()
        .map(|(i, (x, y))| (x, y, i))
        .collect();

    let mut tree = Vec::new();
    make_tree(&mut seq, &mut tree, 0, 0, n);

    let q = get!(usize);
    for _ in 0..q {
        let (sx, tx, sy, ty) = get!(usize, usize, usize, usize);
        let mut v: Vec<usize> = Vec::new();
        find(0, 0, &seq, &tree, sx, sy, tx, ty, &mut v);
        v.sort();
        for x in v {
            println!("{}", x);
        }
        println!("");
    }
}
