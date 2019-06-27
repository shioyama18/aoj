// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_8_C

type Link<T> = Option<Box<Node<T>>>;

struct Node<T: Ord> {
    item: T,
    left: Link<T>,
    right: Link<T>,
}

struct Tree<T: Ord> {
    root: Link<T>,
}

impl<T: Ord> Node<T> {
    fn new(x: T) -> Node<T> {
        Node {item: x, left: None, right: None }
    }

    fn insert(&mut self, x: T) {
        // if x == self.item { return; }
        let node = if x < self.item {
            &mut self.left 
        } else {
            &mut self.right
        };
        match node {
            &mut Some(ref mut node1) => node1.insert(x),
            &mut None => {
                *node = Some(Box::new(Node::new(x)))
            }
        }
    }

    fn find(&self, x: T) -> bool {
        if x == self.item { return true; }
        let node = if x < self.item {
            &self.left
        } else {
            &self.right
        };
        match node {
            &Some(ref node1) => node1.find(x),
            &None => false
        }
    }
            
}

impl<T: Ord> Tree<T> {
    fn new() -> Tree<T> {
        Tree { root: None }
    }

    fn insert(&mut self, x: T) {
        if self.root.is_none() {
            self.root = Some(Box::new(Node::new(x)));
        } else {
            self.root.as_mut().map(|node| node.insert(x));
        }
    }

    fn find(&self, x: T) -> bool {
        self.root.as_ref().map_or_else(|| false, |node| node.find(x))
    }

    fn delete(&mut self, x: T) -> Option<T> {
        delete(search_node(&mut self.root, x))
    }

}

fn search_node<T: Ord>(xs: &mut Link<T>, x: T) -> &mut Link<T> {
    match xs {
        &mut Some(ref mut ys) if x != ys.item => {
            if x < ys.item {
                search_node(&mut ys.left, x)
            } else {
                search_node(&mut ys.right, x)
            }
        },
        _ => xs
    }
}

fn search_min_node<T: Ord>(xs: &mut Link<T>) -> &mut Link<T> {
    match xs {
        &mut Some(ref mut node) if node.left.is_some() => search_min_node(&mut node.left),
        _ => xs
    }
}

fn delete<T: Ord>(xs: &mut Link<T>) -> Option<T> {
    if let Some(mut boxed_node) =  xs.take() {
        match (boxed_node.left.take(), boxed_node.right.take()) {
            (None, None) => Some(boxed_node.item),
            (leaf @ Some(_), None) | (None, leaf @ Some(_)) => {
                *xs = leaf;
                Some(boxed_node.item)
            },
            (left, right) => {
                boxed_node.left = left; // Rollback
                boxed_node.right = right;
                let result;
                {
                    let node = &mut *boxed_node;
                    let min_node = search_min_node(&mut node.right);
                    std::mem::swap(&mut node.item, &mut min_node.as_mut().unwrap().item);
                    result = delete(min_node);
                }
                *xs = Some(boxed_node);
                result
            }
        }
    } else {
        None
    }
}

fn inorder(xs: &Link<isize>) {
    match xs {
        &Some(ref node) => {
            inorder(&node.left);
            print!(" {}", node.item);
            inorder(&node.right);
        },
        _ => ()
    }
}

fn preorder(xs: &Link<isize>) {
    match xs {
        &Some(ref node) => {
            print!(" {}", node.item);
            preorder(&node.left);
            preorder(&node.right);
        },
        _ => ()
    }
}

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).ok();
    let n = n.trim().parse::<usize>().unwrap();

    let mut a: Tree<isize> = Tree::new();
    for _ in 0..n {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).ok();
        let buffer: Vec<&str> = buffer.split_whitespace().collect();
        if buffer[0] == "insert" {
            let x = buffer[1].parse::<isize>().unwrap();
            a.insert(x);
        } else if buffer[0] == "print" {
            inorder(&a.root);
            print!("\n");
            preorder(&a.root);
            print!("\n");
        } else if buffer[0] == "find" {
            let x = buffer[1].parse::<isize>().unwrap();
            if a.find(x) {
                println!("yes");
            } else {
                println!("no");
            }
        } else if buffer[0] == "delete" {
            let x = buffer[1].parse::<isize>().unwrap();
            a.delete(x);
        }
    }
}
