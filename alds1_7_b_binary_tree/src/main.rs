// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_7_B

type NodeId = isize;

#[derive(Clone)]
struct Node {
    parent: NodeId,
    right: NodeId,
    left: NodeId,
}

fn set_depth(nodes: &Vec<Node>, depth: &mut Vec<usize>, u: usize, d: usize) {
    depth[u] = d;
    if nodes[u].left != -1 {
        set_depth(nodes, depth, nodes[u].left as usize, d + 1);
    }
    if nodes[u].right!= -1 {
        set_depth(nodes, depth, nodes[u].right as usize, d + 1);
    }
}

fn set_height(nodes: &Vec<Node>, height: &mut Vec<usize>, u: usize) -> usize {
    let mut h1 = 0;
    let mut h2 = 0;
    
    if nodes[u].left != -1 {
        h1 = set_height(nodes, height, nodes[u].left as usize) + 1;
    }
    if nodes[u].right != -1 {
        h2 = set_height(nodes, height, nodes[u].right as usize) + 1;
    }
    height[u] = std::cmp::max(h1, h2);
    height[u]
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
    
    let mut depth: Vec<usize> = vec![0; n];
    let mut height: Vec<usize> = vec![0; n];

    set_depth(&nodes, &mut depth, root, 0);
    set_height(&nodes, &mut height, root);

    for i in 0..n {
        print!("node {}: parent = {}, ", i, nodes[i].parent);
        
        // print siblings
        if nodes[i].parent == -1 {
            print!("sibling = -1, ");
        } else if nodes[nodes[i].parent as usize].left != i as isize {
            print!("sibling = {}, ", nodes[nodes[i].parent as usize].left); 
        } else {
            print!("sibling = {}, ", nodes[nodes[i].parent as usize].right);             
        }

        // print degree
        let mut degree = 0;
        if nodes[i].left != -1 { degree += 1; }
        if nodes[i].right != -1 { degree += 1; }
        print!("degree = {}, ", degree);


        // print depth and height
        print!("depth = {}, height = {}, ", depth[i], height[i]);

        // print type
        if nodes[i].parent == -1 {
            println!("root");
        } else if nodes[i].left == -1 && nodes[i].right == -1 {
            println!("leaf");
        } else {
            println!("internal node");
        }
    }
    
}
