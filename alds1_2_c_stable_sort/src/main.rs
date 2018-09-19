// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_2_C

#[derive(Clone)]
struct Card {
    suit: String,
    value: u32,
}

fn is_stable(v1: &Vec<Card>, v2: &Vec<Card>) -> bool {
    for (i, j) in v1.iter().zip(v2.iter()) {
        if i.suit != j.suit {
            return false
        }
    }
    true
}

fn print_result(v: &Vec<Card>) {
    for (i, val) in v.iter().enumerate() {
        if i > 0 { print!(" "); }
        print!("{}{}", val.suit, val.value);
    }
    print!("\n");
}    

fn bubble_sort(v_orig: &Vec<Card>) -> Vec<Card> {
    let mut v: Vec<Card> = v_orig.clone().to_vec();

    for i in 0..v.len() {
        for j in (i+1..v.len()).rev() {
            if v[j].value < v[j-1].value {
                v.swap(j-1, j);
            }
        }
    }
    v
}

fn selection_sort(v_orig: &Vec<Card>) -> Vec<Card> {
    let mut v: Vec<Card> = v_orig.clone().to_vec();
    
    for i in 0..v.len() {
        let mut minj = i;
        for j in i..v.len() {
            if v[j].value < v[minj].value {
                minj = j;
            }
        }
        v.swap(i, minj);
    }
    v
 }

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let v: Vec<Card> = 
        buffer.split_whitespace()
        .map(|s| Card {
            suit: s[0..1].to_string(),
            value: s[1..2].parse::<u32>().unwrap(),
        })
        .collect();

    let v_bubble: Vec<Card> = bubble_sort(&v);
    let v_selection: Vec<Card> = selection_sort(&v);

    print_result(&v_bubble);
    println!("Stable");
    print_result(&v_selection);
    if is_stable(&v_selection, &v_bubble) {
        println!("Stable");
    } else {
        println!("Not stable");
    }
}
