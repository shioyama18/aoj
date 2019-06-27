// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_8_A

enum Tree {
    Empty,
    Node(isize, Box<Tree>, Box<Tree>),
}

impl Tree {
    fn new() -> Self {
        Tree::Empty
    }

    fn insert(&mut self, value: isize) -> &mut Self {
        match *self {
            Tree::Empty => {
                *self = Tree::Node(value, Box::new(Tree::Empty), Box::new(Tree::Empty))
            },
            Tree::Node(ref old_value, ref mut left, ref mut right) => {
                if &value < old_value {
                    left.insert(value);
                } else {
                    right.insert(value);
                }
            }
        }
        self
    }

    fn inorder(&self) {
        match *self {
            Tree::Empty => (),
            Tree::Node(ref value, ref left, ref right) => {
                left.inorder();
                print!(" {}", *value);
                right.inorder();
            }
        }
    }

    fn preorder(&self) {
        match *self {
            Tree::Empty => (),
            Tree::Node(ref value, ref left, ref right) => {
                print!(" {}", *value);
                left.preorder();
                right.preorder();
            }
        }
    }
}

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).ok();
    let n = n.trim().parse::<usize>().unwrap();

    let mut root = Tree::new();
    for _ in 0..n {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).ok();
        let buffer: Vec<&str> = buffer.split_whitespace().collect();
        if buffer[0] == "insert" {
            let x = buffer[1].parse::<isize>().unwrap();
            root.insert(x);
        } else if buffer[0] == "print" {
            root.inorder();
            print!("\n");
            root.preorder();
            print!("\n");
        }
    }
}
