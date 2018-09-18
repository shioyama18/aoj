// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_11_D

// // recursive version causes stack overflow with large number of nodes
// fn dfs(u: usize, id: usize, m: &Vec<Vec<usize>>, group_id: &mut Vec<usize>) {
//     group_id[u] = id;
//     for v in m[u].iter() {
//         if group_id[*v] == 0 { dfs(u, id, m, group_id); }
//     }
// }

fn dfs(u: usize, id: usize, m: &Vec<Vec<usize>>, group_id: &mut Vec<usize>) {
    let mut s = Vec::new();
    s.push(u);
    group_id[u] = id;
    while let Some(x) = s.pop() {
        for v in m[x].iter() {
            if group_id[*v] == 0 {
                group_id[*v] = id;
                s.push(*v);
            }
        }
    }
}

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).ok();
    let n: Vec<usize> = n.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let num_nodes = n[0];
    let num_edges = n[1];

    let mut m: Vec<Vec<usize>> = vec![Vec::new(); num_nodes];
    for _ in 0..num_edges {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).ok();
        let buffer: Vec<usize> = buffer.split_whitespace().map(|x| x.parse().unwrap()).collect();
        
        m[buffer[0]].push(buffer[1]);
        m[buffer[1]].push(buffer[0]);
    }
    
    let mut group_id = vec![0; num_nodes];

    // assign group id to nodes
    let mut id = 1;
    for u in 0..num_nodes {
        if group_id[u] == 0 { dfs(u, id, &m, &mut group_id); }
        id += 1;
    }

    let mut n = String::new();
    std::io::stdin().read_line(&mut n).ok();
    let n = n.trim().parse::<usize>().unwrap();

    for _ in 0..n { 
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).ok();
        let buffer: Vec<usize> = buffer.split_whitespace().map(|x| x.parse().unwrap()).collect();

        let start = buffer[0];
        let end = buffer[1];

        if group_id[start] == group_id[end] {
            println!("yes");
        } else {
            println!("no");
        }
    }
    
}
