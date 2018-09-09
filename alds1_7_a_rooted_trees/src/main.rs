// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_7_A

type NodeId = usize;

#[derive(Clone, Debug)]
struct Node {
    parent: Option<isize>,
    left: Option<NodeId>,
    right: Option<NodeId>,
}

fn fill_depth(nodes: &Vec<Node>, depth: &mut Vec<usize>, u: usize, p: usize) {
    depth[u] = p;
    if nodes[u].right.is_some() { fill_depth(nodes, depth, nodes[u].right.unwrap(), p); }
    if nodes[u].left.is_some() { fill_depth(nodes, depth, nodes[u].left.unwrap(), p + 1); }
}

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).ok();
    let n = n.trim().parse::<usize>().unwrap();

    let mut nodes: Vec<Node> = vec![Node { parent: None, left: None, right: None }; n];

    let mut left = 0;
    for _ in 0..n {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).ok();
        let buffer: Vec<NodeId> = buffer.split_whitespace()
            .map(|x| x.parse::<NodeId>().unwrap())
            .collect();
        let node_id = buffer[0];
        let degree = buffer[1];
        for j in 0..degree {
            if j == 0 {
                nodes[node_id].left = Some(buffer[2]);
            } else {
                nodes[left].right = Some(buffer[2 + j]);
            }
            left = buffer[2 + j];
            nodes[buffer[2 + j]].parent = Some(node_id as isize);
        }
    }

    let mut r = 0;
    for i in 0..n {
        match nodes[i].parent {
            Some(_) => (),
            None => r = i,
        }
    }

    let mut depth: Vec<usize> = vec![0; n];
    fill_depth(&nodes, &mut depth, r, 0);

    for i in 0..n {
        print!("node {}: parent = {}, depth = {}, ", i, nodes[i].parent.unwrap_or(-1), depth[i]);
        if nodes[i].parent.is_none() {
            print!("root, ");
        } else if nodes[i].left.is_none() {
            print!("leaf, ");
        } else {
            print!("internal node, ");
        }
        let mut children: Vec<usize> = Vec::new();
        let mut c = nodes[i].left;
        while let Some(j) = c {
            children.push(j);
            c = nodes[j].right;
        }

        println!("{:?}", children);
    }
}
