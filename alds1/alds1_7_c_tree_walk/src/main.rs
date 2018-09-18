// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_7_C

type NodeId = isize;

#[derive(Clone, Debug)]
struct Node {
    parent: NodeId,
    left: NodeId,
    right: NodeId,
}

fn print_preorder(nodes: &Vec<Node>, i: usize) {
    print!(" {}", i);
    if nodes[i].left != -1 {
        print_preorder(nodes, nodes[i].left as usize);
    }
    if nodes[i].right != -1 {
        print_preorder(nodes, nodes[i].right as usize);
    }
}

fn print_inorder(nodes: &Vec<Node>, i: usize) {
    if nodes[i].left != -1 {
        print_inorder(nodes, nodes[i].left as usize);
    }
    print!(" {}", i);
    if nodes[i].right != -1 {
        print_inorder(nodes, nodes[i].right as usize);
    }
}

fn print_postorder(nodes: &Vec<Node>, i: usize) {
    if nodes[i].left != -1 {
        print_postorder(nodes, nodes[i].left as usize);
    }
    if nodes[i].right != -1 {
        print_postorder(nodes, nodes[i].right as usize);
    }
    print!(" {}", i);
}

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).ok();
    let n = n.trim().parse::<usize>().unwrap();

    let mut nodes: Vec<Node> = vec![Node { parent: -1, left: -1, right: -1 }; n];

    for _ in 0..n {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).ok();
        let buffer: Vec<NodeId> = buffer.split_whitespace()
            .map(|x| x.parse::<NodeId>().unwrap())
            .collect();

        nodes[buffer[0] as usize].left = buffer[1];
        nodes[buffer[0] as usize].right = buffer[2];
        if buffer[1] != -1 {
            nodes[buffer[1] as usize].parent = buffer[0];
        }
        if buffer[2] != -1 {
            nodes[buffer[2] as usize].parent = buffer[0];
        }
    }

    let mut root = 0;
    for i in 0..n {
        if nodes[i].parent == -1 {
            root = i;
        }
    }

    println!("Preorder");
    print_preorder(&nodes, root);
    print!("\n");
    println!("Inorder");
    print_inorder(&nodes, root);
    print!("\n");
    println!("Postorder");
    print_postorder(&nodes, root);
    print!("\n");
}
