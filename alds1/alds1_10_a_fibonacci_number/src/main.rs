// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_10_A

fn fib(n: usize, memo: &mut Vec<usize>) -> usize {
    if memo[n] != 0 {
        memo[n]
    } else if n <= 1 {
        1
    } else {
        memo[n] = fib(n-1, memo) + fib(n-2, memo);
        memo[n]
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
    let input = input.trim().parse::<usize>().unwrap();
    
    let mut memo = vec![0; input+1];
    println!("{}", fib(input, &mut memo));
}
