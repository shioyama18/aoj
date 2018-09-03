use std::collections::VecDeque;

struct Process {
    pid: String,
    t: u32,
}    

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let input: Vec<u32> = n.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let n = input[0];
    let q = input[1];
    
    let mut queue: VecDeque<Process> = VecDeque::new();
    for _ in 0..n {
        let mut entry = String::new();
        std::io::stdin().read_line(&mut entry).unwrap();
        let input: Vec<&str> = entry.split_whitespace().collect();
        queue.push_back(Process { pid: input[0].to_string(), t: input[1].parse().unwrap() });
    }
        
    let mut time = 0;
    while !queue.is_empty() {
        let mut p = queue.pop_front().unwrap();
        let c = std::cmp::min(q, p.t);
        p.t -= c;
        time += c;
        if p.t == 0 {
            println!("{} {}", p.pid, time);
        } else {
            queue.push_back(p);
        }
    }
}
