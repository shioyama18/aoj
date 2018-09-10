// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_7_D

fn reconstruct(preorder: &Vec<usize>, inorder: &Vec<usize>, postorder: &mut Vec<usize>, pos: &mut usize, l: usize, r: usize) {
    if l < r {
        let root = preorder[*pos];
        *pos += 1;
        let m = inorder.iter().position(|&x| x == root).unwrap();
        reconstruct(preorder, inorder, postorder, pos, l, m);
        reconstruct(preorder, inorder, postorder, pos, m + 1, r);
        postorder.push(root);
    }
}


fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).ok();
    let n = n.trim().parse::<usize>().unwrap();

    let mut preorder = String::new();
    std::io::stdin().read_line(&mut preorder).ok();
    let preorder: Vec<usize> = preorder.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut inorder = String::new();
    std::io::stdin().read_line(&mut inorder).ok();
    let inorder: Vec<usize> = inorder.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut postorder: Vec<usize> = Vec::with_capacity(n);
    let mut pos = 0;
    
    reconstruct(&preorder, &inorder, &mut postorder, &mut pos, 0, n);

    println!("{}", postorder.iter()
             .map(|x| x.to_string())
             .collect::<Vec<String>>()
             .join(" "));
}
